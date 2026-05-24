use dioxus::prelude::*;

use crate::address::{NetAddress, parse_mask};

#[component]
pub fn HostCount() -> Element {
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "Nombre d'hôtes" }
            p { "Calcule le nombre d'adresses d'hôtes disponibles pour un masque donné." }

            input {
                class: "{input_class()}",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| { mask.set(e.value().clone()); is_valid.set(true); }
            }
            button {
                class: "action-button",
                onclick: move |_| {
                    match parse_mask(&mask.read()) {
                        Ok(mask_val) => {
                            is_valid.set(true);
                            let count = NetAddress::new(0, mask_val).host_count();
                            let explanation = match mask_val {
                                31 => "2 (RFC 3021 : liaison point à point)".to_string(),
                                32 => "1 (adresse unique)".to_string(),
                                _ => {
                                    let total = 2u32.pow(32 - mask_val);
                                    format!("{} hôtes disponibles\n({} adresses totales − 2)", count, total)
                                }
                            };
                            result.set(explanation);
                        }
                        Err(e) => {
                            is_valid.set(false);
                            result.set(e.to_string());
                        }
                    }
                },
                "Calculer"
            }
            p { class: "result", "{result.read()}" }
        }
    }
}
