use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut show_modal = use_signal(|| false);

    rsx! {
        div {
            class: "tool-container",
            h1 { "Bienvenue dans NetCalc-RS !" }
            p { "Explorez les concepts fondamentaux de l'adressage IP via des outils visuels interactifs." }

            button {
                class: "action-button",
                onclick: move |_| show_modal.set(true),
                "📘 Théorie Réseau"
            }

            if *show_modal.read() {
                div {
                    class: "modal-overlay",
                    onclick: move |_| show_modal.set(false), // Ferme si clic sur overlay
                    div {
                        class: "modal-content",
                        onclick: |_| {}, // Empêche propagation
                        h2 { "📘 Théorie : Concepts clés" }
                        ul {
                            li { strong { "IPv4 vs IPv6: " } "IPv4 utilise 32 bits (4.3 milliards d'adresses), IPv6 utilise 128 bits (3.4×10^38)."
                                br {}
                                strong { "🧩 IPv4 : " }
                                "192.168.1.1 (adresse privée classique)"
                                br {}
                                strong { "🧩 IPv6 : " }
                                "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
                                br {}
                                em { "→ Forme raccourcie : 2001:db8:85a3::8a2e:370:7334" }
                            }
                            li { strong { "Classes IP: " } "Historiques : Classe A (grands réseaux), B, C (petits). Aujourd’hui remplacées par CIDR." }
                            li { strong { "CIDR (/n): " } "Permet de définir précisément la taille d’un réseau. Ex: /24 = 256 adresses." 
                                br {}
                                "192.168.1.0/24"
                                br {}
                                em { "→ masque décimal équivalent : 255.255.255.0" }
                            }
                            li { strong { "Pourquoi subnetter ? " } "Pour optimiser l’allocation des adresses IP, séparer les zones réseau logiquement."
                                br {}
                                strong { "🔀 Exemple de sous-réseaux : " }
                                "192.168.1.0/26, 192.168.1.64/26, 192.168.1.128/26, etc."
                                br {}
                                em { "→ Issus d’un découpage de 192.168.1.0/24 en /26" }
                            }
                        }
                        button {
                            class: "action-button",
                            onclick: move |_| show_modal.set(false),
                            "Fermer"
                        }
                    }
                }
            }
        }
    }
}

