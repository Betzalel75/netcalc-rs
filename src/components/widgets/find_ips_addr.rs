use crate::{address::NetAddress, components::widgets::utils::BitLine};
use dioxus::prelude::*;
use std::net::Ipv4Addr;
#[allow(non_snake_case)]
#[component]
pub fn FindIpsAddr() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_first_ip = use_signal(String::new);
    let mut binary_last_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);
    let mut err = use_signal(|| false);
    

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
                    if let (Ok(ip_val), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        if mask_val > 32 {
                            result.set("Entrées invalides".to_string());
                            err.set(true);
                        }else
                        if mask_val == 32{
                            result.set("Un masque /32 utilise tous les 32 bits pour le réseau. Il n'y a aucun bit disponible pour l'hôte".to_string());
                        }else{
                        let ip_octets: Vec<u32> = ip_val.octets().iter().map(|b| *b as u32).collect();
                        let ip_u32 = (ip_octets[0] << 24) | (ip_octets[1] << 16) | (ip_octets[2] << 8) | ip_octets[3];

                        let (first, last) = NetAddress::ip_range(ip_u32, mask_val);

                        result.set(format!(
                            "Première IP utilisable : {}\nDernière IP utilisable : {}",
                            crate::components::format_ipv4(first),
                            crate::components::format_ipv4(last),
                        ));

                        binary_first_ip.set(format!("{:032b}", first));
                        binary_last_ip.set(format!("{:032b}", last));
                        binary_mask.set(format!("{:032b}", 0xFFFFFFFFu32 << (32 - mask_val)));
                    }

                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_first_ip.set("".to_string());
                        binary_last_ip.set("".to_string());
                        binary_mask.set("".to_string());
                        err.set(true);
                    }
                },
                "Calculer"
            }
            
            if *err.read() || result.read().is_empty(){
                p{
                    class: "result",
                    "Pour calculer la première et la dernière adresse utilisable d'un réseau à partir de son adresse réseau et de son masque, suivez ces étapes :"
                    ul { 
                        li { 
                            "Calculer la première adresse utilisable :
                            La première adresse utilisable est l'adresse qui suit immédiatement l'adresse réseau.
                            Pour l'obtenir, prenez l'adresse réseau et incrémentez le dernier octet (ou l'octet le moins significatif) de 1."
                         }
                        li { 
                            "Calculer la dernière adresse utilisable :
                            La dernière adresse utilisable est l'adresse qui précède immédiatement l'adresse de diffusion.
                            Pour l'obtenir, prenez l'adresse de diffusion et décrémentez le dernier octet de 1."
                         }
                        li { 
                            "En résumé :
                            Première adresse utilisable = Adresse réseau + 1
                            Dernière adresse utilisable = Adresse de diffusion - 1"
                         }
                     }
                }
            }

            pre { class: "result", "{result.read()}" }

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
