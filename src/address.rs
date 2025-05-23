pub struct NetAddress {
    pub address: u32,
    pub mask: u32,
}

impl NetAddress {
    pub fn new(address: u32, mask: u32) -> Self {
        Self { address, mask }
    }

    fn calculate_number_of_network_addresses(&self, nouveau_masque: u32) -> u32 {
        2u32.pow((nouveau_masque - self.mask) as u32)
    }

    pub fn ip_range(adresse: u32, masque: u32) -> (u32, u32) {
        let nombre_ips = 2u32.pow(32 - masque);
        let premiere = if nombre_ips < 2 { adresse } else { adresse + 1 };
        let derniere = if masque < 31 {
            adresse + nombre_ips - 2
        } else {
            adresse + 2
        };
        (premiere, derniere)
    }

    pub fn calcmask(nb_adresses_ip: u32) -> u32 {
        if nb_adresses_ip < 1 {
            println!("Erreur valeur invalid!");
            return 0;
        }
        32 - (nb_adresses_ip as f32).log2().ceil() as u32
    }

    pub fn subnet_split(&self, nouveau_masque: u32) -> Vec<NetAddress> {
        let nombre_sous_reseaux = self.calculate_number_of_network_addresses(nouveau_masque);
        let taille_sous_reseau = 2u32.pow(32 - nouveau_masque);
        let mut sous_reseaux = Vec::new();

        // On aligne l'adresse sur le masque original
        let adresse_base = self.address & (0xFFFFFFFFu32 << (32 - self.mask));

        for i in 0..nombre_sous_reseaux {
            let adresse = adresse_base + i * taille_sous_reseau;
            sous_reseaux.push(NetAddress::new(adresse, nouveau_masque));
        }

        sous_reseaux
    }

    pub fn broadcast_address(&self) -> u32 {
        self.address | ((1u32 << (32 - self.mask)) - 1)
    }

    pub fn number_of_host_addresses(masque: u32) -> u32 {
        if masque >= 32 || masque <= 0 {
            return 0;
        }
        if masque == 31 {
            return 2;
        }
        2u32.pow(32 - masque) - 2
    }
    pub fn to_binary_string(&self) -> String {
        format!(
            "{:08b}.{:08b}.{:08b}.{:08b}",
            (self.address >> 24) & 255,
            (self.address >> 16) & 255,
            (self.address >> 8) & 255,
            self.address & 255
        )
    }
    pub fn ip_to_string(&self) -> String {
        String::from(format!(
            "{}.{}.{}.{}/{}",
            (self.address >> 24) & 255,
            (self.address >> 16) & 255,
            (self.address >> 8) & 255,
            self.address & 255,
            self.mask
        ))
    }
}

impl std::fmt::Display for NetAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            // {:?}
            write!(
                f,
                "{}.{}.{}.{}/{}\n<=>{}",
                (self.address >> 24) & 255,
                (self.address >> 16) & 255,
                (self.address >> 8) & 255,
                self.address & 255,
                self.mask,
                self.to_binary_string()
            )
        } else {
            write!(
                f,
                "{}.{}.{}.{}/{}",
                (self.address >> 24) & 255,
                (self.address >> 16) & 255,
                (self.address >> 8) & 255,
                self.address & 255,
                self.mask
            )
        }
    }
}
