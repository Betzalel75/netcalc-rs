use std::net::Ipv4Addr;
use dioxus::prelude::*;

use crate::address::{format_ipv4, NetAddress};

#[component]
pub fn Broadcast() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Adresse de diffusion (broadcast)" }
            p { "Calcule l'adresse de diffusion à partir d'une adresse IP et d'un masque." }

            input {
                class: "{input_class()}",
                placeholder: "Adresse IP (ex: 192.168.1.10)",
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
                                let broadcast = net.broadcast_address();
                                result.set(format!("Adresse de diffusion : {}", format_ipv4(broadcast)));
                            }
                            Err(e) => {
                                is_valid.set(false);
                                result.set(e.to_string());
                            }
                        }
                    } else {
                        is_valid.set(false);
                        result.set("Entrées invalides".to_string());
                    }
                },
                "Calculer"
            }
            p { class: "result", "{result.read()}" }
        }
    }
}
