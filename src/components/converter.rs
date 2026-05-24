use std::net::Ipv4Addr;
use dioxus::prelude::*;
use crate::address::{format_ipv4, format_ipv4_hex, u32_to_binary_string, ipv4_to_u32};

#[component]
pub fn Converter() -> Element {
    let mut ip_input = use_signal(String::new);
    let mut decimal = use_signal(String::new);
    let mut binary = use_signal(String::new);
    let mut hex = use_signal(String::new);
    let mut integer = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    rsx! {
        div { class: "tool-container",
            h3 { "Convertisseur de formats IP" }
            p { "Convertit une IPv4 entre les formats décimal, binaire, hexadécimal et entier 32 bits." }

            input {
                class: if *is_valid.read() { "input-field" } else { "input-field invalid" },
                placeholder: "Adresse IPv4 (ex: 192.168.1.1)",
                value: "{ip_input}",
                oninput: move |e| {
                    ip_input.set(e.value().clone());
                    if let Ok(ip) = e.value().parse::<Ipv4Addr>() {
                        let ip_u32 = ipv4_to_u32(ip);
                        decimal.set(format_ipv4(ip_u32));
                        binary.set(u32_to_binary_string(ip_u32));
                        hex.set(format_ipv4_hex(ip_u32));
                        integer.set(ip_u32.to_string());
                        is_valid.set(true);
                    } else {
                        is_valid.set(false);
                    }
                }
            }

            if *is_valid.read() && !decimal.read().is_empty() {
                div { class: "result",
                    table { style: "width: 100%; margin-top: 0;",
                        tr { td { strong { "Décimal" } } td { style: "font-family: monospace; font-size: 1.1rem;", "{decimal.read()}" } }
                        tr { td { strong { "Binaire" } } td { style: "font-family: monospace; font-size: 1.1rem;", "{binary.read()}" } }
                        tr { td { strong { "Hexadécimal" } } td { style: "font-family: monospace; font-size: 1.1rem;", "{hex.read()}" } }
                        tr { td { strong { "Entier 32 bits" } } td { style: "font-family: monospace; font-size: 1.1rem;", "{integer.read()}" } }
                    }
                }
            } else if !*is_valid.read() && !ip_input.read().is_empty() {
                p { class: "result error", "Adresse IPv4 invalide." }
            }
        }
    }
}
