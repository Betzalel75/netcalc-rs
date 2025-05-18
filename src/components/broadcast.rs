use std::net::Ipv4Addr;

use dioxus::prelude::*;

use crate::{address::NetAddress, components::format_ipv4};

#[component]
pub fn Broadcast() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "Adresse de diffusion" }
            input { class: "input-field", placeholder: "Adresse IP", oninput: move |e| ip.set(e.value().clone()) }
            input { class: "input-field", placeholder: "Masque", oninput: move |e| mask.set(e.value().clone()) }
            button { class: "action-button", onclick: move |_| {
                if let (Ok(ip_addr), Ok(mask_val)) = (ip.read().parse::<Ipv4Addr>(), mask.read().parse::<u32>()) {
                    let ip_vec: Vec<u32> = ip_addr.octets().iter().map(|&b| b as u32).collect();
                    let ip_u32 = (ip_vec[0] << 24) | (ip_vec[1] << 16) | (ip_vec[2] << 8) | ip_vec[3];
                    let net = NetAddress::new(ip_u32, mask_val);
                    let addr = net.determiner_adresse_diffusion();
                    result.set(format_ipv4(addr));
                } else {
                    result.set("Entrées invalides".to_string());
                }
            }, "Calculer" }
            p { class: "result", "{result.read()}" }
        }
    }
}
