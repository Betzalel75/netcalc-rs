use std::net::Ipv4Addr;
use dioxus::prelude::*;
use crate::{components::widgets::utils::{BitLine, BitLines}, SCRIPT};
#[allow(non_snake_case)]


#[component]
pub fn CalculatorNetAddress() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);
    let mut binary_network = use_signal(String::new);
    let mut animate = use_signal(||false);

    rsx! {
        div { class: "tool-container",
            h3 { "🧮 Calcul de l'adresse réseau" }
            p { "Saisissez une adresse IP et un masque pour visualiser le calcul binaire (opération AND bit à bit)." }

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
                        if mask_val > 32 || mask_val < 1  {
                            animate.set(false);
                            result.set("Entrées invalides".to_string());
                            binary_ip.set("".to_string());
                            binary_mask.set("".to_string());
                            binary_network.set("".to_string());
                        }else{
                            let ip_octets: Vec<u32> = ip_val.octets().iter().map(|b| *b as u32).collect();
                            let ip_u32 = (ip_octets[0] << 24) | (ip_octets[1] << 16) | (ip_octets[2] << 8) | ip_octets[3];
                            let mask_u32 = 0xFFFFFFFFu32 << (32 - mask_val);
                            let network = ip_u32 & mask_u32;

                            result.set(format!("Adresse Réseau : {}.{}.{}.{}",
                                (network >> 24) & 255,
                                (network >> 16) & 255,
                                (network >> 8) & 255,
                                network & 255,
                            ));

                            binary_ip.set(format!("{:032b}", ip_u32));
                            binary_mask.set(format!("{:032b}", mask_u32));
                            binary_network.set(format!("{:032b}", network));
                            animate.set(true); 
                        }
                    } else {
                        animate.set(false);
                        result.set("Entrées invalides".to_string());
                        binary_ip.set("".to_string());
                        binary_mask.set("".to_string());
                        binary_network.set("".to_string());
                    }
                },
                "Calculer"
            }

            div {
                style: "margin-top: 1rem;",
                p { "🧠 Représentation binaire :" }

                div {
                    style: "font-family: monospace; white-space: pre-wrap;",
                    BitLine { label: "IP".to_string(), bits: binary_ip.read().clone(), color: "#29b0b0".to_string() }
                    BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "orange".to_string() }
                    BitLine { label: "Résultat".to_string(), bits: binary_network.read().clone(), color: "limegreen".to_string() }
                    if *animate.read(){
                    BitLines{label: "".to_string(), bits: binary_ip.read().clone(), line: 1}
                    BitLines{label: "".to_string(), bits: binary_mask.read().clone(), line: 2}
                    BitLines{label: "".to_string(), bits: binary_network.read().clone(), line: 3}
                        script { 
                            script { src:SCRIPT }
                        }
                    }
                }
            }

            p { class: "result", "{result.read()}" }
        }
    }
}
