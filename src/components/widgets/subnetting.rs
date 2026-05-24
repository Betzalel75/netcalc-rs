use std::net::Ipv4Addr;
use dioxus::prelude::*;
use crate::{
    address::NetAddress,
    components::widgets::utils::Tables,
    display_table::Table,
};

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
                    if let (Ok(ip_addr), Ok(mask_val), Ok(new_mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>(),
                        new_mask.read().parse::<u32>()
                    ) {
                        match NetAddress::from_ip_and_mask(ip_addr, mask_val) {
                            Ok(net) => match net.subnet_split(new_mask_val) {
                                Ok(subnets) => {
                                    err.set(false);
                                    let mut table = Table::new();
                                    table.headers = vec![
                                        "IP".to_string(),
                                        "Binaire".to_string(),
                                    ];
                                    for addr in &subnets {
                                        table.add_row(&[
                                            addr.to_cidr_string(),
                                            addr.to_binary_string(),
                                        ]);
                                    }
                                    final_table.set(table);
                                }
                                Err(_) => {
                                    err.set(true);
                                    final_table.set(Table::new());
                                }
                            },
                            Err(_) => {
                                err.set(true);
                                final_table.set(Table::new());
                            }
                        }
                    } else {
                        err.set(true);
                        final_table.set(Table::new());
                    }
                },
                "Découper"
            }

            if final_table.read().body.is_empty() || *err.read() {
                if *err.read() {
                    pre { class: "result", "Entrées invalides" }
                } else {
                    pre { class: "result", "Aucun sous-réseau à afficher" }
                }
            } else {
                Tables { table: final_table.read().clone() }
            }
        }
    }
}
