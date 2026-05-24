use dioxus::prelude::*;
use std::net::Ipv4Addr;

use crate::address::{format_ipv4, format_ipv4_hex, ipv4_to_u32, NetAddress, u32_to_binary_string, parse_cidr_flexible};
use crate::components::app::History;

#[component]
pub fn Home() -> Element {
    let mut ip_input = use_signal(String::new);
    let mut net_info = use_signal(|| None::<DashboardInfo>);
    let mut error = use_signal(String::new);
    let mut is_valid = use_signal(|| true);
    let history = use_context::<History>();

    rsx! {
        div { class: "home-container",
            h1 { "NetCalc-RS" }
            p { class: "subtitle", "Calculateur d'adressage IP : Tableau de bord" }

            div { class: "dashboard-input",
                input {
                    class: if *is_valid.read() { "input-field" } else { "input-field invalid" },
                    placeholder: "Adresse IP/CIDR (ex: 192.168.1.0/24)",
                    value: "{ip_input}",
                    oninput: move |e| { ip_input.set(e.value().clone()); is_valid.set(true); }
                }
                button {
                    class: "action-button",
                    onclick: move |_| {
                        let ip = ip_input.read().trim().to_string();
                        
                        if let Ok((ip_addr, mask_val)) = parse_cidr_flexible(&ip) {
                            match NetAddress::from_ip_and_mask(ip_addr, mask_val) {
                                Ok(net) => {
                                    is_valid.set(true);
                                    error.set("".to_string());
                                    let (first, last) = net.ip_range();
                                    let history_cidr = net.to_cidr_string();
                                    {
                                        let mut h = history;
                                        h.write().push(history_cidr);
                                        if h.read().len() > 5 {
                                            h.write().remove(0);
                                        }
                                    }
                                    net_info.set(Some(DashboardInfo {
                                        cidr: net.to_cidr_string(),
                                        network: format_ipv4(net.network_address()),
                                        broadcast: format_ipv4(net.broadcast_address()),
                                        subnet_mask: format_ipv4(net.subnet_mask()),
                                        wildcard: format_ipv4(net.wildcard_mask()),
                                        first_host: format_ipv4(first),
                                        last_host: format_ipv4(last),
                                        host_count: net.host_count(),
                                        total_addresses: net.total_addresses(),
                                        mask_val: net.mask,
                                        binary_mask: u32_to_binary_string(net.subnet_mask()),
                                        binary_ip: u32_to_binary_string(ipv4_to_u32(ip_addr)),
                                        hex_ip: format_ipv4_hex(ipv4_to_u32(ip_addr)),
                                        ip_class: classify_ip(ip_addr),
                                    }));
                                }
                                Err(e) => {
                                    is_valid.set(false);
                                    error.set(e.to_string());
                                    net_info.set(None);
                                }
                            }
                        } else if let Ok(ip_addr) = ip.parse::<Ipv4Addr>() {
                            is_valid.set(true);
                            error.set("".to_string());
                            net_info.set(Some(DashboardInfo {
                                cidr: format_ipv4(ipv4_to_u32(ip_addr)),
                                network: "-".to_string(),
                                broadcast: "-".to_string(),
                                subnet_mask: "-".to_string(),
                                wildcard: "-".to_string(),
                                first_host: "-".to_string(),
                                last_host: "-".to_string(),
                                host_count: 0,
                                total_addresses: 0,
                                mask_val: 0,
                                binary_mask: "-".to_string(),
                                binary_ip: u32_to_binary_string(ipv4_to_u32(ip_addr)),
                                hex_ip: format_ipv4_hex(ipv4_to_u32(ip_addr)),
                                ip_class: classify_ip(ip_addr),
                            }));
                        } else {
                            is_valid.set(false);
                            error.set("Format invalide. Exemples : 192.168.1.0/24, 10.0.0.1 255.0.0.0".to_string());
                            net_info.set(None);
                        }
                    },
                    "Analyser"
                }
            }

            if !error.read().is_empty() {
                p { class: "result error", "{error.read()}" }
            }

            if let Some(info) = &*net_info.read() {
                div { class: "dashboard-grid",
                    div { class: "dashboard-card",
                        h3 { "🔗 Informations réseau" }
                        div { class: "info-row",
                            span { class: "info-label", "Adresse réseau" }
                            span { class: "info-value", "{info.network}" }
                            CopyButton { text: info.network.clone() }
                        }
                        div { class: "info-row",
                            span { class: "info-label", "Broadcast" }
                            span { class: "info-value", "{info.broadcast}" }
                            CopyButton { text: info.broadcast.clone() }
                        }
                        div { class: "info-row",
                            span { class: "info-label", "Masque" }
                            span { class: "info-value", "{info.subnet_mask}" }
                            CopyButton { text: info.subnet_mask.clone() }
                        }
                        if info.mask_val > 0 {
                            div { class: "info-row",
                                span { class: "info-label", "CIDR" }
                                span { class: "info-value", "/{info.mask_val}" }
                                CopyButton { text: info.cidr.clone() }
                            }
                        }
                        div { class: "info-row",
                            span { class: "info-label", "Wildcard" }
                            span { class: "info-value", "{info.wildcard}" }
                            CopyButton { text: info.wildcard.clone() }
                        }
                    }
                    div { class: "dashboard-card",
                        h3 { "💻 Adresses & Hôtes" }
                        div { class: "info-row",
                            span { class: "info-label", "Première IP" }
                            span { class: "info-value", "{info.first_host}" }
                            CopyButton { text: info.first_host.clone() }
                        }
                        div { class: "info-row",
                            span { class: "info-label", "Dernière IP" }
                            span { class: "info-value", "{info.last_host}" }
                            CopyButton { text: info.last_host.clone() }
                        }
                        InfoRow { label: "Hôtes".to_string(), value: format!("{}", info.host_count) }
                        InfoRow { label: "Total adr.".to_string(), value: format!("{}", info.total_addresses) }
                        InfoRow { label: "Classe IP".to_string(), value: info.ip_class.clone() }
                    }
                    div { class: "dashboard-card",
                        h3 { "🔢 Conversions" }
                        InfoRow { label: "Décimal".to_string(), value: info.cidr.clone() }
                        InfoRow { label: "Hexadécimal".to_string(), value: info.hex_ip.clone() }
                        div { class: "info-row",
                            span { class: "info-label", "Binaire" }
                            span { class: "info-value binary", "{info.binary_ip}" }
                            CopyButton { text: info.binary_ip.clone() }
                        }
                        div { class: "info-row",
                            span { class: "info-label", "Masque bin." }
                            span { class: "info-value binary", "{info.binary_mask}" }
                        }
                    }
                }

                if info.mask_val > 0 {
                    div { class: "dashboard-card binary-breakdown",
                        h3 { "🧬 Décomposition binaire de l'adresse réseau" }
                        BitGrid {
                            binary_ip: info.binary_ip.clone(),
                            binary_mask: info.binary_mask.clone(),
                            mask_val: info.mask_val,
                        }
                    }
                }
            } else if ip_input.read().is_empty() {
                div { class: "welcome-tips",
                    div { class: "dashboard-card",
                        h3 { "Bienvenue !" }
                        p { "Saisissez une adresse IP avec son masque CIDR pour afficher toutes les informations du sous-réseau." }
                        p { class: "hint", "Exemples : 192.168.1.0/24, 10.0.0.1/8, 172.16.0.0/12" }
                    }
                }
            }
        }
    }
}

// ── Data ────────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct DashboardInfo {
    cidr: String,
    network: String,
    broadcast: String,
    subnet_mask: String,
    wildcard: String,
    first_host: String,
    last_host: String,
    host_count: u32,
    total_addresses: u64,
    mask_val: u32,
    binary_mask: String,
    binary_ip: String,
    hex_ip: String,
    ip_class: String,
}

fn classify_ip(ip: Ipv4Addr) -> String {
    let octets = ip.octets();
    let first = octets[0];

    if ip.is_loopback() { return "Boucle locale (127.0.0.0/8)".to_string(); }
    if ip.is_private() {
        return match first {
            10 => "10.0.0.0/8 (privé)".to_string(),
            172 if (16..=31).contains(&octets[1]) => "172.16.0.0/12 (privé)".to_string(),
            192 if octets[1] == 168 => "192.168.0.0/16 (privé)".to_string(),
            _ => "Adresse privée".to_string(),
        };
    }
    if ip.is_multicast() { return "Classe D (Multicast)".to_string(); }
    if first >= 240 { return "Classe E (Expérimental)".to_string(); }

    match first {
        0..=127 => "Classe A".to_string(),
        128..=191 => "Classe B".to_string(),
        192..=223 => "Classe C".to_string(),
        _ => "Inconnue".to_string(),
    }
}

// ── Components ──────────────────────────────────────────────────────────────────

#[component]
fn InfoRow(label: String, value: String) -> Element {
    rsx! {
        div { class: "info-row",
            span { class: "info-label", "{label}" }
            span { class: "info-value", "{value}" }
        }
    }
}

/// Affiche les 32 bits de l'IP avec un code couleur :
/// - fond bleu pour les bits du réseau (là où le masque vaut 1)
/// - fond transparent pour les bits d'hôte (là où le masque vaut 0)
#[component]
fn BitGrid(binary_ip: String, binary_mask: String, mask_val: u32) -> Element {
    // Aplatir les chaînes : retirer les points
    let ip_flat: String = binary_ip.chars().filter(|c| *c != '.').collect();
    let mask_flat: Vec<bool> = binary_mask
        .chars()
        .filter(|c| *c != '.')
        .map(|c| c == '1')
        .collect();

    let host_bits = 32 - mask_val;

    rsx! {
        div { class: "bit-grid-container",
            // Ligne des bits IP
            div { class: "bit-grid",
                for (i, ch) in ip_flat.chars().enumerate() {
                    span {
                        class: if mask_flat[i] { "bit-cell network" } else { "bit-cell host" },
                        "{ch}"
                    }
                    // Espace entre les octets
                    if i % 8 == 7 && i < 31 {
                        span { class: "bit-gap" }
                    }
                }
            }
            // Ligne des bits masque
            div { class: "bit-grid mask",
                for (i, is_net) in mask_flat.iter().enumerate() {
                    span {
                        class: if *is_net { "bit-cell network" } else { "bit-cell host" },
                        {if *is_net { "1" } else { "0" }}
                    }
                    if i % 8 == 7 && i < 31 {
                        span { class: "bit-gap" }
                    }
                }
            }
            div { class: "bit-legend",
                span { class: "legend-item network",
                    "█ Partie réseau : {mask_val} bits"
                }
                span { class: "legend-item host",
                    "░ Partie hôte : {host_bits} bits"
                }
            }
        }
    }
}

/// Bouton qui copie un texte dans le presse-papier.
#[component]
fn CopyButton(text: String) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "copy-btn",
            title: "Copier dans le presse-papier",
            onclick: move |_| {
                let escaped = text.replace('\\', "\\\\").replace('\'', "\\'");
                let js = format!("navigator.clipboard.writeText('{}')", escaped);
                let _ = dioxus::document::eval(&js);
                copied.set(true);
            },
            if *copied.read() {
                "✓"
            } else {
                "📋"
            }
        }
    }
}
