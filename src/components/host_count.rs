use dioxus::prelude::*;

use crate::address::NetAddress;

#[component]
pub fn HostCount() -> Element {
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "Nombre d'hôtes" }
            input { class: "input-field", placeholder: "Masque", oninput: move |e| mask.set(e.value().clone()) }
            button { class: "action-button", onclick: move |_| {
                if let Ok(mask_val) = mask.read().parse::<u32>() {
                    let count = NetAddress::nombre_adresses_hotes(mask_val);
                    let val = if mask_val == 31 {
                        "2 (liaison point à point entre deux routeurs. \
                            Bien qu'il y ait deux adresses IP, elles ne sont pas considérées\
                            comme des adresses 'hôtes' disponibles au sens traditionnel\
                            pour des ordinateurs ou des périphériques finaux.\
                            Le RFC 3021 permet l'utilisation de ces deux adresses pour la liaison,\
                            sans adresse de réseau ni de diffusion distinctes.)".to_string()
                    }else{
                        count.to_string()
                    };
                    result.set(val);
                } else {
                    result.set("Entrée invalide".to_string());
                }
            }, "Calculer" }
            p { class: "result", "{result.read()}" }
        }
    }
}
