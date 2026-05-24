use std::net::Ipv4Addr;
use dioxus::prelude::*;

use crate::address::{format_ipv4, ipv4_to_u32, NetAddress};

#[component]
pub fn IpChecker() -> Element {
    let mut network = use_signal(String::new);
    let mut check_ip = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut is_in_subnet = use_signal(|| false);
    let mut has_result = use_signal(|| false);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Vérificateur : IP dans sous-réseau ?" }
            p { "Vérifie si une adresse IP donnée appartient à un sous-réseau." }

            input { class: "{input_class()}", placeholder: "Sous-réseau (ex: 192.168.1.0/24)", value: "{network}",
                oninput: move |e| { network.set(e.value().clone()); is_valid.set(true); } }
            input { class: "{input_class()}", placeholder: "IP à vérifier (ex: 192.168.1.50)", value: "{check_ip}",
                oninput: move |e| { check_ip.set(e.value().clone()); is_valid.set(true); } }

            button {
                class: "action-button",
                onclick: move |_| {
                    match NetAddress::from_str(&network.read()) {
                        Ok(net) => {
                            match check_ip.read().trim().parse::<Ipv4Addr>() {
                                Ok(ip) => {
                                    let ip_u32 = ipv4_to_u32(ip);
                                    let inside = net.contains(ip_u32);
                                    is_valid.set(true);
                                    has_result.set(true);
                                    is_in_subnet.set(inside);
                                    result.set(if inside {
                                        format!("{} appartient au sous-réseau {}", format_ipv4(ip_u32), net.to_cidr_string())
                                    } else {
                                        format!("{} n'appartient PAS au sous-réseau {}", format_ipv4(ip_u32), net.to_cidr_string())
                                    });
                                }
                                Err(_) => { is_valid.set(false); has_result.set(false); result.set("Adresse IP invalide.".to_string()); }
                            }
                        }
                        Err(e) => { is_valid.set(false); has_result.set(false); result.set(e.to_string()); }
                    }
                },
                "Vérifier"
            }

            if *has_result.read() {
                p {
                    class: if *is_in_subnet.read() { "result success" } else { "result error" },
                    "{result.read()}"
                }
            } else if !result.read().is_empty() {
                p { class: "result error", "{result.read()}" }
            }
        }
    }
}
