use dioxus::prelude::*;
use std::net::Ipv4Addr;

use crate::address::{format_ipv4, NetAddress};

#[component]
pub fn IpRange() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut network = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Plage d'IP disponible" }
            p { "Calcule la première et dernière adresse utilisable d'un sous-réseau." }

            input {
                class: "{input_class()}",
                placeholder: "Adresse IP (ex: 192.168.1.0)",
                value: "{ip}",
                oninput: move |e| { ip.set(e.value().clone()); is_valid.set(true); }
            }
            input {
                class: "{input_class()}",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| { mask.set(e.value().clone()); is_valid.set(true); }
            }
            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_addr), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        match NetAddress::from_ip_and_mask(ip_addr, mask_val) {
                            Ok(net) => {
                                is_valid.set(true);
                                let net_addr = net.network_address();
                                let (first, last) = net.ip_range();
                                if mask_val == 32 {
                                    result.set("Masque /32 : une seule adresse, pas de plage utilisable.".to_string());
                                    network.set("".to_string());
                                } else {
                                    network.set(format!("Adresse réseau : {}", format_ipv4(net_addr)));
                                    result.set(format!(
                                        "Première IP utilisable : {}\nDernière IP utilisable : {}",
                                        format_ipv4(first), format_ipv4(last)
                                    ));
                                }
                            }
                            Err(e) => {
                                is_valid.set(false);
                                result.set(e.to_string());
                                network.set("".to_string());
                            }
                        }
                    } else {
                        is_valid.set(false);
                        result.set("Entrées invalides".to_string());
                        network.set("".to_string());
                    }
                },
                "Calculer"
            }

            if !network.read().is_empty() {
                p { class: "result network", "{network.read()}" }
            }
            p { class: "result", "{result.read()}" }
        }
    }
}
