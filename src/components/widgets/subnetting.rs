use std::net::Ipv4Addr;
use dioxus::prelude::*;
use crate::{address::NetAddress, components::widgets::utils::Tables, display_table::Table};
#[allow(non_snake_case)]


#[component]
pub fn Subnetting() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut new_mask = use_signal(String::new);
    let mut final_table = use_signal(|| Table::new());
    let mut err = use_signal(|| false);

    rsx! {
        div { class: "tool-container",
            h3 { "🔀 Découpage en sous-réseaux" }
            p { "Divise un réseau en plusieurs sous-réseaux à l'aide d'un masque plus précis." }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.0)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque actuel (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Nouveau masque (ex: 26)",
                value: "{new_mask}",
                oninput: move |e| new_mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val), Ok(new_mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>(),
                        new_mask.read().parse::<u32>()
                    ) {
                        if mask_val > 32 || mask_val < 1 || new_mask_val <= mask_val || new_mask_val > 32 {
                            err.set(true);
                        }else{
                            err.set(false);
                            let ip_u32 = u32::from(ip_val);
                            let net = NetAddress::new(ip_u32, mask_val);

                            let sous_reseaux = net.subnet_split(new_mask_val);
                            let mut table = Table::new();
                            table.headers = vec![String::from("IP"), String::from("Binaire")];

                            for addr in &sous_reseaux {
                                table.add_row(&[
                                    addr.ip_to_string(),
                                    addr.to_binary_string(),
                                ]);
                            }
                            final_table.set(table); 
                        }
                    }else{
                        err.set(true);
                    }
                },
                "Découper"
            }
            if final_table.read().body.is_empty() || *err.read() {
                if *err.read(){
                    pre {
                        class: "result",
                        "Entrées invalides"
                    }
                }else{
                    pre {
                        class: "result",
                        "Aucun sous-réseau à afficher"
                    }
                }
            } else {
                Tables { table: final_table.read().clone() }
            }
        }
    }
}
