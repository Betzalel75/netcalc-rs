use dioxus::prelude::*;
use crate::address::{format_ipv4, parse_cidr};

#[component]
pub fn Wildcard() -> Element {
    let mut input = use_signal(String::new);
    let mut wildcard = use_signal(String::new);
    let mut mask_str = use_signal(String::new);
    let mut prefix = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Wildcard Mask (masque inverse)" }
            p { "Calcule le wildcard mask utilisé dans les ACL Cisco et OSPF/EIGRP." }
            p { class: "hint", "Le wildcard mask est le complément à 1 du masque de sous-réseau." }

            input { class: "{input_class()}", placeholder: "CIDR (ex: 192.168.1.0/24) ou masque seul (ex: 24)", value: "{input}",
                oninput: move |e| { input.set(e.value().clone()); is_valid.set(true); } }

            button {
                class: "action-button",
                onclick: move |_| {
                    let val = input.read().trim().to_string();
                    if let Ok((_ip, mask)) = parse_cidr(&val) {
                        let subnet = if mask == 0 { 0 } else { 0xFFFFFFFFu32 << (32 - mask) };
                        let wc = 0xFFFFFFFFu32 ^ subnet;
                        wildcard.set(format_ipv4(wc));
                        mask_str.set(format_ipv4(subnet));
                        prefix.set(format!("/{}", mask));
                        is_valid.set(true);
                        return;
                    }
                    if let Ok(mask) = val.parse::<u32>() {
                        if mask <= 32 {
                            let subnet = if mask == 0 { 0 } else { 0xFFFFFFFFu32 << (32 - mask) };
                            let wc = 0xFFFFFFFFu32 ^ subnet;
                            wildcard.set(format_ipv4(wc));
                            mask_str.set(format_ipv4(subnet));
                            prefix.set(format!("/{}", mask));
                            is_valid.set(true);
                            return;
                        }
                    }
                    if let Ok(ip) = val.parse::<std::net::Ipv4Addr>() {
                        let mask_u32 = u32::from(ip);
                        let wc = 0xFFFFFFFFu32 ^ mask_u32;
                        wildcard.set(format_ipv4(wc));
                        mask_str.set(format_ipv4(mask_u32));
                        prefix.set(format!("/{}", mask_u32.count_ones()));
                        is_valid.set(true);
                        return;
                    }
                    is_valid.set(false);
                    wildcard.set("".to_string());
                    mask_str.set("".to_string());
                    prefix.set("".to_string());
                },
                "Calculer"
            }

            if !wildcard.read().is_empty() {
                div { class: "result",
                    div { style: "display: flex; gap: 2rem; align-items: center;",
                        div {
                            p { strong { "Masque de sous-réseau" } }
                            p { style: "font-size: 1.2rem; font-family: monospace;", "{mask_str.read()} ({prefix.read()})" }
                        }
                        div {
                            p { strong { "Wildcard Mask" } }
                            p { style: "font-size: 1.2rem; font-family: monospace; color: var(--color-accent);", "{wildcard.read()}" }
                        }
                    }
                    p { class: "hint", "Formule : Wildcard = 255.255.255.255 − Masque" }
                    p { class: "hint", "Usage : ACL Cisco, OSPF network, EIGRP wildcard bits" }
                }
            } else if !*is_valid.read() {
                p { class: "result error", "Entrée invalide. Exemples : 24, 255.255.255.0, 192.168.1.0/24" }
            }
        }
    }
}
