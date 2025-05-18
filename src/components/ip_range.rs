use std::net::Ipv4Addr;

use dioxus::prelude::*;

use crate::{address::NetAddress, components::format_ipv4};

#[component]
pub fn IpRange() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "Plage d'IP disponible" }
            input {
                class: "input-field",
                placeholder: "Adresse IP",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }
            input {
                class: "input-field",
                placeholder: "Masque",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }
            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_addr), Ok(mask_val)) = (ip.read().parse::<Ipv4Addr>(), mask.read().parse::<u32>()) {
                        let ip_u32 = u32::from(ip_addr);
                        let (first, last) = NetAddress::determiner_plage_ip(ip_u32, mask_val);
                        let res = format!("Première IP: {} | Dernière IP: {}", format_ipv4(first), format_ipv4(last));
                        result.set(res);
                    } else {
                        result.set("Entrées invalides".to_string());
                    }
                },
                "Calculer"
            }
            p { class: "result", "{result.read()}" }
        }
    }
}
