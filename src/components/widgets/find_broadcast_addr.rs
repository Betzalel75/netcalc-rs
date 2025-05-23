use std::net::Ipv4Addr;
use dioxus::prelude::*;
use crate::{address::NetAddress, components::widgets::utils::BitLine};
#[allow(non_snake_case)]


#[component]
pub fn FindBroadcastAddr() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);
    let mut binary_broadcast = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "📡 Adresse de diffusion" }
            p { "Calcule l'adresse de diffusion à partir d'une IP et d'un masque. (IP OR complément du masque)" }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.10)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        let ip_u32 = u32::from(ip_val);
                        let broadcast = NetAddress::new(ip_u32, mask_val).broadcast_address();

                        result.set(format!("Adresse de diffusion : {}", crate::components::format_ipv4(broadcast)));

                        binary_ip.set(format!("{:032b}", ip_u32));
                        binary_mask.set(format!("{:032b}", 0xFFFFFFFFu32 << (32 - mask_val)));
                        binary_broadcast.set(format!("{:032b}", broadcast));
                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_ip.set("".to_string());
                        binary_mask.set("".to_string());
                        binary_broadcast.set("".to_string());
                    }
                },
                "Calculer"
            }

            p { class: "result", "{result.read()}" }

             div {
                style: "margin-top: 1rem;",
                p { "🧠 Représentation binaire :" }

                div {
                    style: "font-family: monospace; white-space: pre-wrap;",
                    BitLine { label: "IP".to_string(), bits: binary_ip.read().clone(), color: "cyan".to_string() }
                    BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "orange".to_string() }
                    BitLine { label: "Diffusion".to_string(), bits: binary_broadcast.read().clone(), color: "limegreen".to_string() }
                }
            }
        }
    }
}
