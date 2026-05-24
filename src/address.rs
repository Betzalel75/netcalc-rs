use std::fmt;
use std::net::Ipv4Addr;

// ─── Error types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum NetError {
    InvalidMask(u32),
    InvalidIp(String),
    InvalidCidr(String),
    InvalidNewMask { current: u32, new: u32 },
    InvalidHostCount(u32),
    EmptyRouteList,
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::InvalidMask(m) => write!(f, "Masque invalide : /{} (doit être entre 1 et 32)", m),
            NetError::InvalidIp(s) => write!(f, "Adresse IP invalide : {}", s),
            NetError::InvalidCidr(s) => write!(f, "Notation CIDR invalide : {}", s),
            NetError::InvalidNewMask { current, new } => write!(
                f,
                "Le nouveau masque /{} doit être strictement supérieur au masque actuel /{}",
                new, current
            ),
            NetError::InvalidHostCount(n) => write!(f, "Nombre d'hôtes invalide : {} (doit être >= 1)", n),
            NetError::EmptyRouteList => write!(f, "La liste de routes est vide"),
        }
    }
}

// ─── NetAddress ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetAddress {
    pub address: u32,
    pub mask: u32,
}

impl NetAddress {
    /// Constructeur non validé — usage interne uniquement.
    /// Préférer `from_str`, `from_ip_and_mask` ou `try_new` pour les données externes.
    pub fn new(address: u32, mask: u32) -> Self {
        Self { address, mask }
    }

    /// Constructeur validé.
    pub fn try_new(address: u32, mask: u32) -> Result<Self, NetError> {
        if mask < 1 || mask > 32 {
            return Err(NetError::InvalidMask(mask));
        }
        Ok(Self { address, mask })
    }

    /// Depuis une chaîne CIDR (ex: "192.168.1.0/24").
    pub fn from_str(cidr: &str) -> Result<Self, NetError> {
        let (ip, mask) = parse_cidr(cidr)?;
        Self::try_new(ipv4_to_u32(ip), mask)
    }

    /// Depuis une IPv4 et un masque.
    pub fn from_ip_and_mask(ip: Ipv4Addr, mask: u32) -> Result<Self, NetError> {
        Self::try_new(ipv4_to_u32(ip), mask)
    }

    // ── Propriétés calculées ──────────────────────────────────────────────────

    /// Adresse réseau (IP & masque).
    pub fn network_address(&self) -> u32 {
        self.address & subnet_mask_u32(self.mask)
    }

    /// Adresse de diffusion (IP | ~masque).
    pub fn broadcast_address(&self) -> u32 {
        let host_bits = (1u64 << (32 - self.mask)) - 1;
        (self.network_address() as u64 | host_bits) as u32
    }

    /// Plage d'adresses utilisables (première, dernière).
    /// Retourne (0, 0) pour /32 car aucune adresse utilisable.
    pub fn ip_range(&self) -> (u32, u32) {
        let network = self.network_address();
        let broadcast = self.broadcast_address();

        if self.mask >= 31 {
            // /31 : les deux adresses sont utilisables (RFC 3021)
            // /32 : une seule adresse, pas de plage
            if self.mask == 32 {
                return (network, network);
            }
            return (network, broadcast);
        }

        (network + 1, broadcast - 1)
    }

    /// Nombre d'adresses d'hôtes disponibles.
    pub fn host_count(&self) -> u32 {
        match self.mask {
            0 => 0,
            31 => 2, // RFC 3021
            32 => 1, // adresse unique
            m => 2u32.pow(32 - m) - 2,
        }
    }

    /// Masque de sous-réseau (représentation u32).
    pub fn subnet_mask(&self) -> u32 {
        subnet_mask_u32(self.mask)
    }

    /// Wildcard mask (complément du masque, utilisé pour les ACL).
    pub fn wildcard_mask(&self) -> u32 {
        !self.subnet_mask()
    }

    /// Vérifie si une IP appartient à ce sous-réseau.
    pub fn contains(&self, ip: u32) -> bool {
        let network = self.network_address();
        let broadcast = self.broadcast_address();
        ip >= network && ip <= broadcast
    }

    /// Nombre total d'adresses dans le sous-réseau (inclut réseau et broadcast).
    pub fn total_addresses(&self) -> u64 {
        if self.mask == 0 {
            return 4_294_967_296; // 2^32
        }
        2u64.pow(32 - self.mask)
    }

    /// Découpage en sous-réseaux avec un masque plus fin.
    pub fn subnet_split(&self, new_mask: u32) -> Result<Vec<NetAddress>, NetError> {
        if new_mask <= self.mask || new_mask > 32 {
            return Err(NetError::InvalidNewMask {
                current: self.mask,
                new: new_mask,
            });
        }

        let subnet_count = 2u32.pow((new_mask - self.mask) as u32);
        let subnet_size = 2u32.pow(32 - new_mask);
        let base = self.network_address();

        let subnets: Vec<NetAddress> = (0..subnet_count)
            .map(|i| NetAddress::new(base + i * subnet_size, new_mask))
            .collect();

        Ok(subnets)
    }

    // ── Conversions en chaînes ────────────────────────────────────────────────

    /// Représentation binaire (32 bits par groupes de 8).
    pub fn to_binary_string(&self) -> String {
        u32_to_binary_string(self.address)
    }

    /// Notation CIDR (ex: "192.168.1.0/24").
    pub fn to_cidr_string(&self) -> String {
        format!("{}/{}", format_ipv4(self.address), self.mask)
    }

    /// Notation hexadécimale.
    pub fn to_hex_string(&self) -> String {
        u32_to_hex_string(self.address)
    }
}

impl fmt::Display for NetAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}/{}\n<=>{}",
                format_ipv4(self.address),
                self.mask,
                self.to_binary_string()
            )
        } else {
            write!(f, "{}/{}", format_ipv4(self.address), self.mask)
        }
    }
}

// ── Fonctions statiques ────────────────────────────────────────────────────────

/// Calcule le masque CIDR minimal pour un nombre d'hôtes donné.
/// Prend en compte les adresses réseau et broadcast.
pub fn calcmask(host_count: u32) -> Result<u32, NetError> {
    if host_count < 1 {
        return Err(NetError::InvalidHostCount(host_count));
    }
    // On a besoin de host_count + 2 adresses (réseau + broadcast)
    let total_ips = host_count.saturating_add(2);
    let bits = 32 - (total_ips as f64).log2().ceil() as u32;
    Ok(bits)
}

/// Agrégation de routes (supernetting / route summarization).
/// Trouve le plus petit super-réseau contenant toutes les routes données.
pub fn summarize(routes: &[NetAddress]) -> Result<NetAddress, NetError> {
    if routes.is_empty() {
        return Err(NetError::EmptyRouteList);
    }

    let mut min_ip = u32::MAX;
    let mut max_ip = 0u32;

    for route in routes {
        let first = route.network_address();
        let last = route.broadcast_address();
        if first < min_ip {
            min_ip = first;
        }
        if last > max_ip {
            max_ip = last;
        }
    }

    // Trouver le masque commun
    let range = max_ip - min_ip;
    let host_bits = if range == 0 {
        0
    } else {
        32 - range.leading_zeros()
    };
    let mask = 32 - host_bits;

    // Aligner l'adresse de base sur le masque
    let base = min_ip & subnet_mask_u32(mask);

    Ok(NetAddress::new(base, mask))
}

/// Vérifie si deux sous-réseaux se chevauchent.
pub fn overlap(a: &NetAddress, b: &NetAddress) -> bool {
    let a_net = a.network_address();
    let a_bc = a.broadcast_address();
    let b_net = b.network_address();
    let b_bc = b.broadcast_address();

    a_net <= b_bc && b_net <= a_bc
}

// ── Utilitaires de conversion ──────────────────────────────────────────────────

/// Convertit une IPv4 en u32.
pub fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

/// Convertit un u32 en IPv4.
pub fn u32_to_ipv4(ip: u32) -> Ipv4Addr {
    Ipv4Addr::from(ip)
}

/// Formate un u32 en notation décimale pointée.
pub fn format_ipv4(ip: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}

/// Formate un u32 en notation hexadécimale.
pub fn format_ipv4_hex(ip: u32) -> String {
    format!(
        "{:02X}.{:02X}.{:02X}.{:02X}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}

/// Convertit un u32 en chaîne binaire (groupes de 8 bits).
pub fn u32_to_binary_string(ip: u32) -> String {
    format!(
        "{:08b}.{:08b}.{:08b}.{:08b}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}

/// Convertit un u32 en chaîne hexadécimale (groupes de 2 octets).
pub fn u32_to_hex_string(ip: u32) -> String {
    format!(
        "{:02X}.{:02X}.{:02X}.{:02X}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}

/// Génère le masque de sous-réseau (u32) à partir d'une longueur de préfixe.
pub fn subnet_mask_u32(mask: u32) -> u32 {
    if mask == 0 {
        0
    } else {
        0xFFFFFFFFu32 << (32 - mask)
    }
}

/// Parse une chaîne CIDR (ex: "192.168.1.0/24") en (Ipv4Addr, u32).
pub fn parse_cidr(input: &str) -> Result<(Ipv4Addr, u32), NetError> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 2 {
        return Err(NetError::InvalidCidr(input.to_string()));
    }

    let ip = parts[0]
        .trim()
        .parse::<Ipv4Addr>()
        .map_err(|_| NetError::InvalidIp(parts[0].to_string()))?;

    let mask = parts[1]
        .trim()
        .parse::<u32>()
        .map_err(|_| NetError::InvalidCidr(input.to_string()))?;

    if mask > 32 {
        return Err(NetError::InvalidMask(mask));
    }

    Ok((ip, mask))
}

/// Parse une adresse IP seule (sans masque).
pub fn parse_ip(input: &str) -> Result<Ipv4Addr, NetError> {
    input
        .trim()
        .parse::<Ipv4Addr>()
        .map_err(|_| NetError::InvalidIp(input.to_string()))
}

/// Parse un masque (notation CIDR, ex: "24") en u32.
pub fn parse_mask(input: &str) -> Result<u32, NetError> {
    let mask = input
        .trim()
        .parse::<u32>()
        .map_err(|_| NetError::InvalidMask(0))?;

    if mask > 32 {
        return Err(NetError::InvalidMask(mask));
    }

    Ok(mask)
}

/// Parse une IP + masque séparés.
pub fn parse_ip_and_mask(ip_str: &str, mask_str: &str) -> Result<(Ipv4Addr, u32), NetError> {
    let ip = parse_ip(ip_str)?;
    let mask = parse_mask(mask_str)?;
    Ok((ip, mask))
}

/// Parse une chaîne CIDR flexible. Accepte :
/// - "192.168.1.0/24"        (CIDR standard)
/// - "192.168.1.0 24"         (IP + espace + masque)
/// - "192.168.1.0/255.255.255.0" (IP / masque décimal)
/// - "192.168.1.0 255.255.255.0" (IP + espace + masque décimal)
pub fn parse_cidr_flexible(input: &str) -> Result<(Ipv4Addr, u32), NetError> {
    let input = input.trim();

    // Essayer "IP/masque_ou_cidr"
    if let Some((ip_part, mask_part)) = input.split_once('/') {
        let ip = parse_ip(ip_part)?;
        // Essayer d'abord comme nombre entier (CIDR)
        if let Ok(mask) = parse_mask(mask_part) {
            return Ok((ip, mask));
        }
        // Sinon essayer comme masque décimal pointé
        if let Ok(dotted) = mask_part.trim().parse::<Ipv4Addr>() {
            let mask = mask_from_ip(dotted);
            return Ok((ip, mask));
        }
        return Err(NetError::InvalidCidr(input.to_string()));
    }

    // Essayer "IP masque_ou_cidr" (séparés par un espace)
    if let Some((ip_part, mask_part)) = input.split_once(char::is_whitespace) {
        let ip = parse_ip(ip_part)?;
        if let Ok(mask) = parse_mask(mask_part.trim()) {
            return Ok((ip, mask));
        }
        if let Ok(dotted) = mask_part.trim().parse::<Ipv4Addr>() {
            let mask = mask_from_ip(dotted);
            return Ok((ip, mask));
        }
        return Err(NetError::InvalidMask(0));
    }

    Err(NetError::InvalidCidr(input.to_string()))
}

/// Calcule la longueur de préfixe à partir d'un masque décimal pointé.
/// Ex: 255.255.255.0 → 24
pub fn mask_from_ip(ip: Ipv4Addr) -> u32 {
    u32::from(ip).count_ones()
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_address() {
        let net = NetAddress::from_str("192.168.1.100/24").unwrap();
        let network = net.network_address();
        assert_eq!(format_ipv4(network), "192.168.1.0");
    }

    #[test]
    fn test_broadcast_address() {
        let net = NetAddress::from_str("192.168.1.0/24").unwrap();
        assert_eq!(format_ipv4(net.broadcast_address()), "192.168.1.255");
    }

    #[test]
    fn test_ip_range() {
        let net = NetAddress::from_str("192.168.1.0/24").unwrap();
        let (first, last) = net.ip_range();
        assert_eq!(format_ipv4(first), "192.168.1.1");
        assert_eq!(format_ipv4(last), "192.168.1.254");
    }

    #[test]
    fn test_ip_range_slash_31() {
        let net = NetAddress::from_str("192.168.1.0/31").unwrap();
        let (first, last) = net.ip_range();
        assert_eq!(format_ipv4(first), "192.168.1.0");
        assert_eq!(format_ipv4(last), "192.168.1.1");
    }

    #[test]
    fn test_ip_range_slash_32() {
        let net = NetAddress::from_str("192.168.1.100/32").unwrap();
        let (first, last) = net.ip_range();
        assert_eq!(format_ipv4(first), "192.168.1.100");
        assert_eq!(format_ipv4(last), "192.168.1.100");
    }

    #[test]
    fn test_host_count() {
        assert_eq!(NetAddress::from_str("0.0.0.0/24").unwrap().host_count(), 254);
        assert_eq!(NetAddress::from_str("0.0.0.0/31").unwrap().host_count(), 2);
        assert_eq!(NetAddress::from_str("0.0.0.0/32").unwrap().host_count(), 1);
        assert_eq!(NetAddress::from_str("0.0.0.0/16").unwrap().host_count(), 65534);
    }

    #[test]
    fn test_subnet_split() {
        let net = NetAddress::from_str("192.168.1.0/24").unwrap();
        let subnets = net.subnet_split(26).unwrap();
        assert_eq!(subnets.len(), 4);
        assert_eq!(subnets[0].to_cidr_string(), "192.168.1.0/26");
        assert_eq!(subnets[1].to_cidr_string(), "192.168.1.64/26");
        assert_eq!(subnets[2].to_cidr_string(), "192.168.1.128/26");
        assert_eq!(subnets[3].to_cidr_string(), "192.168.1.192/26");
    }

    #[test]
    fn test_contains() {
        let net = NetAddress::from_str("192.168.1.0/24").unwrap();
        assert!(net.contains(ipv4_to_u32(Ipv4Addr::new(192, 168, 1, 50))));
        assert!(net.contains(ipv4_to_u32(Ipv4Addr::new(192, 168, 1, 0))));
        assert!(net.contains(ipv4_to_u32(Ipv4Addr::new(192, 168, 1, 255))));
        assert!(!net.contains(ipv4_to_u32(Ipv4Addr::new(192, 168, 2, 1))));
    }

    #[test]
    fn test_wildcard_mask() {
        let net = NetAddress::from_str("192.168.1.0/24").unwrap();
        assert_eq!(format_ipv4(net.wildcard_mask()), "0.0.0.255");
    }

    #[test]
    fn test_calcmask() {
        assert_eq!(calcmask(254).unwrap(), 24);
        assert_eq!(calcmask(50).unwrap(), 26);
        assert_eq!(calcmask(2).unwrap(), 30);
    }

    #[test]
    fn test_summarize() {
        let routes = vec![
            NetAddress::from_str("192.168.1.0/24").unwrap(),
            NetAddress::from_str("192.168.2.0/24").unwrap(),
            NetAddress::from_str("192.168.3.0/24").unwrap(),
        ];
        let summary = summarize(&routes).unwrap();
        assert_eq!(summary.to_cidr_string(), "192.168.0.0/22");
    }

    #[test]
    fn test_overlap() {
        let a = NetAddress::from_str("192.168.1.0/24").unwrap();
        let b = NetAddress::from_str("192.168.1.128/25").unwrap();
        let c = NetAddress::from_str("192.168.2.0/24").unwrap();
        assert!(overlap(&a, &b));
        assert!(!overlap(&a, &c));
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(NetAddress::from_str("not-an-ip").is_err());
        assert!(NetAddress::from_str("192.168.1.0/33").is_err());
        assert!(NetAddress::from_str("192.168.1.0").is_err());
    }
}
