use dioxus::prelude::*;
#[allow(non_snake_case)]

use crate::{address::NetAddress, components::format_ipv4};


#[component]
pub fn SubnetMask() -> Element {
    let mut nb_ips = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div {  class: "tool-container",
            h3 { "Masque de sous-réseau" }
            input { class: "input-field",placeholder: "Nombre d'IPs", oninput: move |e| nb_ips.set(e.value().clone()) }
            button {  class: "action-button", onclick: move |_| {
                if let Ok(nb) = nb_ips.read().parse::<u32>() {
                    let mask = NetAddress::calcmask(nb);
                    let dec_mask = 0xFFFFFFFFu32 << (32 - mask);
                    result.set(format!("/{} => {}", mask, format_ipv4(dec_mask)));
                } else {
                    result.set("Entrée invalide".to_string());
                }
            }, "Calculer" }
            p { class: "result","{result.read()}" }
        }
    }
}
