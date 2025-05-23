use crate::{address::NetAddress, components::widgets::utils::BitLine};
use dioxus::prelude::*;
#[allow(non_snake_case)]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Result{
    count: u32, total_ips: u32, numb: u32, nr: u32
}

impl Result {
    pub fn new()-> Self {
        Result { count:0, total_ips:0, numb:0, nr:0 }
    }
}

#[component]
pub fn FindMask() -> Element {
    let mut nb_ips = use_signal(String::new);
    let mut result = use_signal(|| Result::new());
    let mut binary_mask = use_signal(String::new);
    let mut _mask = use_signal(String::new);


    rsx! {
            div { class: "tool-container",
                h3 { "🛡️ Calcul du masque à partir du nombre d'adresses IP" }
                p { "Indique le masque réseau minimal qui permet de couvrir un nombre donné d'adresses IP." }

                input {
                    class: "input-field",
                    placeholder: "Nombre d'adresses IP (ex: 50)",
                    value: "{nb_ips}",
                    oninput: move |e| nb_ips.set(e.value().clone())
                }

                button {
                    class: "action-button",
                    onclick: move |_| {
                        if let Ok(count) = nb_ips.read().parse::<u32>() {
                            let mask = NetAddress::calcmask(count);
                            let full_mask = 0xFFFFFFFFu32 << (32 - mask);

                            let total_ips = count + 2;
                            let h = (total_ips as f32).log2().ceil() as u32;
                            let nr = 32 - h;
                            _mask.set(format!("/{} → {}", mask, crate::components::format_ipv4(full_mask)));
                            
                            result.set(Result {
                                        count: count,
                                        total_ips: total_ips,
                                        numb: h,
                                         nr: nr, });
                            
                            binary_mask.set(format!("{:032b}", full_mask));
                        } else {
                            binary_mask.set("".to_string());
                        }
                    },
                    "Calculer"
                }

                if _mask.read().is_empty() {
                    p{
                        class: "result",
                        "Pour calculer le masque de réseau à partir du nombre d'hôtes souhaité(N), suivez ces étapes :"
                        ul { 
                            li { 
                                "Déterminez le nombre total d'adresses IP nécessaires.
                                 Si vous avez besoin de N hôtes, le nombre total d'adresses IP nécessaires est N+2.\
                                 Les deux adresses supplémentaires sont l'adresse réseau et l'adresse de diffusion,\
                                 qui ne peuvent pas être attribuées à des hôtes."
                             }
                            li { 
                                "Trouvez le nombre de bits d'hôte nécessaires.
                                    Le nombre de bits d'hôte (h) doit être suffisamment grand pour que 2^h−2≥N.\
                                    En d'autres termes, trouvez le plus petit entier h qui satisfait cette inégalité."
                             }
                            li { 
                                "Déterminez le nombre de bits de réseau.
                                Une adresse IPv4 comporte 32 bits au total.\
                                 Si vous avez h bits pour les hôtes, le nombre de bits pour le réseau (n) sera de n=32−h."
                             }
                         }
                    }
                }else{
                    Explanation {result: *result.read(), mask: _mask.read()},
                }

                if !binary_mask.read().is_empty() {
                    div {
                        style: "margin-top: 1rem;",
                        p { "🧠 Représentation binaire du masque :" }
                        div {
                            style: "font-family: monospace; white-space: pre-wrap;",
                            BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "limegreen".to_string() }
                        }
                    }
                }
            }
        }
}

#[component]
fn Explanation(result: Result, mask: String) -> Element {
    rsx! {
        div { class: "result",
            p {
                "1. Nombre total d'adresses IP nécessaires : "
                b { "{result.count} + 2 = {result.total_ips}." }
                " Les deux adresses supplémentaires sont :"
            }
            ul {
                li { "- L'adresse réseau (non assignable à un hôte)" }
                li { "- L'adresse de diffusion (non assignable à un hôte)" }
            }
            p {
                "2. Nombre de bits d'hôte nécessaires : "
                b { "h = log2({result.total_ips}) " }
                "(arrondir le résultat à l'entier supérieur le plus proche)."
            }
            p {
                "Donc, vous avez besoin de "
                b { "h={result.numb} " }
                "bits d'hôte."
            }
            p {
                "Nombre de bits de réseau : "
                b { "32 - {result.numb} = {result.nr} " }
                "bits."
            }
            p { "{mask}" }
        }
    }
}