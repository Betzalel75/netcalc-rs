use crate::address::{format_ipv4, NetAddress};
use crate::components::widgets::utils::BitLine;
use dioxus::prelude::*;
use std::net::Ipv4Addr;

#[component]
pub fn FindIpsAddr() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_first_ip = use_signal(String::new);
    let mut binary_last_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "📍 Plage d'adresses utilisables" }
            p { "Saisissez une adresse réseau et un masque pour obtenir la première et la dernière adresse IP utilisable du sous-réseau." }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.0)",
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
                    if let (Ok(ip_addr), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        match NetAddress::from_ip_and_mask(ip_addr, mask_val) {
                            Ok(net) => {
                                let (first, last) = net.ip_range();

                                result.set(format!(
                                    "Première IP utilisable : {}\nDernière IP utilisable : {}",
                                    format_ipv4(first),
                                    format_ipv4(last),
                                ));

                                binary_first_ip.set(format!("{:032b}", first));
                                binary_last_ip.set(format!("{:032b}", last));
                                binary_mask.set(format!("{:032b}", net.subnet_mask()));
                            }
                            Err(e) => {
                                result.set(e.to_string());
                                binary_first_ip.set("".to_string());
                                binary_last_ip.set("".to_string());
                                binary_mask.set("".to_string());
                            }
                        }
                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_first_ip.set("".to_string());
                        binary_last_ip.set("".to_string());
                        binary_mask.set("".to_string());
                    }
                },
                "Calculer"
            }

            if result.read().is_empty() {
                p {
                    class: "result",
                    "Pour calculer la première et la dernière adresse utilisable :"
                    ul {
                        li { "Première adresse utilisable = Adresse réseau + 1" }
                        li { "Dernière adresse utilisable = Adresse de diffusion − 1" }
                    }
                }
            }

            pre { class: "result", "{result.read()}" }

            if !binary_first_ip.read().is_empty() {
                div {
                    style: "margin-top: 1rem;",
                    p { "🧠 Représentation binaire :" }
                    div {
                        style: "font-family: monospace; white-space: pre-wrap;",
                        BitLine { label: "Première IP".to_string(), bits: binary_first_ip.read().clone(), color: "cyan".to_string() }
                        BitLine { label: "Dernière IP".to_string(), bits: binary_last_ip.read().clone(), color: "orange".to_string() }
                        BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "limegreen".to_string() }
                    }
                }
            }
        }
    }
}
