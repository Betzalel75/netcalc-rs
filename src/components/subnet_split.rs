use std::net::Ipv4Addr;
#[allow(non_snake_case)]

use dioxus::prelude::*;

use crate::address::NetAddress;



#[component]
pub fn SubnetSplit() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut new_mask = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "Découpe de sous-réseau" }
            input { class: "input-field", placeholder: "Adresse IP", oninput: move |e| ip.set(e.value().clone()) }
            input { class: "input-field", placeholder: "Masque actuel", oninput: move |e| mask.set(e.value().clone()) }
            input { class: "input-field", placeholder: "Nouveau masque", oninput: move |e| new_mask.set(e.value().clone()) }
            button { class: "action-button", onclick: move |_| {
                if let (Ok(ip_addr), Ok(mask_val), Ok(new_mask_val)) = (
                    ip.read().parse::<Ipv4Addr>(),
                    mask.read().parse::<u32>(),
                    new_mask.read().parse::<u32>(),
                ) {
                    let ip_vec: Vec<u32> = ip_addr.octets().iter().map(|&b| b as u32).collect();
                    let ip_u32 = (ip_vec[0] << 24) | (ip_vec[1] << 16) | (ip_vec[2] << 8) | ip_vec[3];
                    let net = NetAddress::new(ip_u32, mask_val);
                    let sous_reseaux = net.subnet_split(new_mask_val);
                    result.set(sous_reseaux.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n"));
                } else {
                    result.set("Entrées invalides".to_string());
                }
            }, "Découper" }
            pre { class: "result", "{result.read()}" }
        }
    }
}
