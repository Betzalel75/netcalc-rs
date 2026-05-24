use std::net::Ipv4Addr;
use dioxus::prelude::*;

use crate::{
    address::{format_ipv4, NetAddress},
    components::widgets::utils::Tables,
    display_table::Table,
};

#[component]
pub fn SubnetSplit() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut new_mask = use_signal(String::new);
    let mut final_table = use_signal(|| Table::new());
    let mut error = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Découpage de sous-réseau (FLSM)" }
            p { "Divise un réseau en sous-réseaux de taille égale avec un masque plus précis." }

            input { class: "{input_class()}", placeholder: "Adresse IP (ex: 192.168.1.0)", value: "{ip}",
                oninput: move |e| { ip.set(e.value().clone()); is_valid.set(true); } }
            input { class: "{input_class()}", placeholder: "Masque actuel (ex: 24)", value: "{mask}",
                oninput: move |e| { mask.set(e.value().clone()); is_valid.set(true); } }
            input { class: "{input_class()}", placeholder: "Nouveau masque (ex: 26)", value: "{new_mask}",
                oninput: move |e| { new_mask.set(e.value().clone()); is_valid.set(true); } }

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
                                    is_valid.set(true);
                                    error.set("".to_string());
                                    let mut table = Table::new();
                                    table.headers = vec![
                                        "Sous-réseau".to_string(),
                                        "Adresse réseau".to_string(),
                                        "Broadcast".to_string(),
                                        "Plage utilisable".to_string(),
                                    ];
                                    for s in subnets.iter() {
                                        let net_addr = format_ipv4(s.network_address());
                                        let bc = format_ipv4(s.broadcast_address());
                                        let (first, last) = s.ip_range();
                                        let range = format!("{} – {}", format_ipv4(first), format_ipv4(last));
                                        table.add_row(&[s.to_cidr_string(), net_addr, bc, range]);
                                    }
                                    final_table.set(table);
                                }
                                Err(e) => { is_valid.set(false); error.set(e.to_string()); final_table.set(Table::new()); }
                            },
                            Err(e) => { is_valid.set(false); error.set(e.to_string()); final_table.set(Table::new()); }
                        }
                    } else {
                        is_valid.set(false);
                        error.set("Entrées invalides".to_string());
                        final_table.set(Table::new());
                    }
                },
                "Découper"
            }

            if !error.read().is_empty() {
                p { class: "result error", "{error.read()}" }
            }
            if !final_table.read().body.is_empty() {
                p { class: "result info", "{final_table.read().body.len()} sous-réseaux générés" }
                Tables { table: final_table.read().clone() }
            }
        }
    }
}
