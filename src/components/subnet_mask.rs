use dioxus::prelude::*;

use crate::address::{calcmask, format_ipv4, subnet_mask_u32};

#[component]
pub fn SubnetMask() -> Element {
    let mut nb_hosts = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Masque de sous-réseau" }
            p { "Détermine le masque CIDR minimal pour accueillir un nombre donné d'hôtes." }

            input {
                class: "{input_class()}",
                placeholder: "Nombre d'hôtes souhaité (ex: 50)",
                value: "{nb_hosts}",
                oninput: move |e| { nb_hosts.set(e.value().clone()); is_valid.set(true); }
            }
            button {
                class: "action-button",
                onclick: move |_| {
                    if let Ok(count) = nb_hosts.read().parse::<u32>() {
                        match calcmask(count) {
                            Ok(cidr_mask) => {
                                is_valid.set(true);
                                let dec_mask = subnet_mask_u32(cidr_mask);
                                let total = count.saturating_add(2);
                                let needed_bits = (total as f64).log2().ceil() as u32;
                                result.set(format!(
                                    "/{} → masque {}\n({} hôtes demandés, {} adresses nécessaires, {} bits d'hôte)",
                                    cidr_mask, format_ipv4(dec_mask), count, total, needed_bits
                                ));
                            }
                            Err(e) => {
                                is_valid.set(false);
                                result.set(e.to_string());
                            }
                        }
                    } else {
                        is_valid.set(false);
                        result.set("Entrée invalide".to_string());
                    }
                },
                "Calculer"
            }
            p { class: "result", "{result.read()}" }
        }
    }
}
