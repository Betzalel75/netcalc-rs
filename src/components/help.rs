use crate::components::widgets::WidgetsPage;
use crate::{
    CISCO_SVG, CONVERTER_SVG, EDUCATION_SVG, GUARD_SVG, NETWORK_GROUP_SVG, NETWORK_SHARE_SVG,
    RULER_SVG, SEARCH_IP_SVG, SUPER_NETTING_SVG, TABLE_SVG, VARIABLE_SVG, WIRELESS_SVG,
};
use dioxus::prelude::*;

#[component]
pub fn Help() -> Element {
    rsx! {
        div { class: "help-container",
            h1 { "Aide & Référence" }
            p { class: "subtitle", "NetCalc-RS : Guide d'utilisation pour ingénieurs réseau et étudiants" }

            // ── Section 1 : Résumé des outils ──
            div { class: "help-section",
                h3 { "📋 Résumé des outils disponibles" }
                div { class: "tool-grid",
                    ToolCard {
                        icon: TABLE_SVG,
                        title: "Tableau de bord",
                        desc: "Analyse complète d'une adresse IP/CIDR : réseau, broadcast, plage, conversions."
                    }
                    ToolCard {
                        icon: RULER_SVG,
                        title: "Plage d'IP",
                        desc: "Affiche la première et dernière adresse utilisable d'un sous-réseau."
                    }
                    ToolCard {
                        icon: GUARD_SVG,
                        title: "Masque → hôtes",
                        desc: "Calcule le masque CIDR minimal pour un nombre d'hôtes donné."
                    }
                    ToolCard {
                        icon: WIRELESS_SVG,
                        title: "Broadcast",
                        desc: "Calcule l'adresse de diffusion d'un sous-réseau."
                    }
                    ToolCard {
                        icon: NETWORK_GROUP_SVG,
                        title: "Nb d'hôtes",
                        desc: "Affiche le nombre d'adresses disponibles pour un masque donné."
                    }
                    ToolCard {
                        icon: NETWORK_SHARE_SVG,
                        title: "Subdivision (FLSM)",
                        desc: "Découpage en sous-réseaux de taille égale avec tableau détaillé."
                    }
                    ToolCard {
                        icon: VARIABLE_SVG,
                        title: "VLSM",
                        desc: "Découpage à taille variable selon les besoins de chaque segment."
                    }
                    ToolCard {
                        icon: SUPER_NETTING_SVG,
                        title: "Agrégation",
                        desc: "Supernetting : regroupe plusieurs routes en un super-réseau."
                    }
                    ToolCard {
                        icon: SEARCH_IP_SVG,
                        title: "IP ∈ sous-réseau",
                        desc: "Vérifie si une adresse IP appartient à un sous-réseau donné."
                    }
                    ToolCard {
                        icon: CISCO_SVG,
                        title: "Wildcard Mask",
                        desc: "Masque inverse pour les ACL Cisco et protocoles de routage."
                    }
                    ToolCard {
                        icon: CONVERTER_SVG,
                        title: "Convertisseur",
                        desc: "Conversion IP entre formats décimal, binaire, hexadécimal, entier."
                    }
                }
            }

            // ── Section 2 : Widgets pédagogiques ──
            div { class: "help-section",
                h3 { span { img {src: EDUCATION_SVG, class: "tool-card-icon" } }, "Widgets pédagogiques interactifs" }
                p { "Explorez ces modules pour comprendre visuellement le calcul d'adressage IP." }
                WidgetsPage {}
            }

            // ── Section 3 : Aide-mémoire ──
            div { class: "help-section",
                h3 { "📝 Aide-mémoire rapide" }
                div { class: "cheatsheet",
                    div { class: "cheatsheet-card",
                        h4 { "Notation CIDR" }
                        table {
                            tr { th { "CIDR" } th { "Masque" } th { "Hôtes" } th { "Total" } }
                            tr { td { "/8" } td { "255.0.0.0" } td { "16 777 214" } td { "16 777 216" } }
                            tr { td { "/16" } td { "255.255.0.0" } td { "65 534" } td { "65 536" } }
                            tr { td { "/24" } td { "255.255.255.0" } td { "254" } td { "256" } }
                            tr { td { "/25" } td { "255.255.255.128" } td { "126" } td { "128" } }
                            tr { td { "/26" } td { "255.255.255.192" } td { "62" } td { "64" } }
                            tr { td { "/27" } td { "255.255.255.224" } td { "30" } td { "32" } }
                            tr { td { "/28" } td { "255.255.255.240" } td { "14" } td { "16" } }
                            tr { td { "/29" } td { "255.255.255.248" } td { "6" } td { "8" } }
                            tr { td { "/30" } td { "255.255.255.252" } td { "2" } td { "4" } }
                            tr { td { "/31" } td { "255.255.255.254" } td { "2" } td { "2 (RFC 3021)" } }
                            tr { td { "/32" } td { "255.255.255.255" } td { "1" } td { "1" } }
                        }
                    }
                    div { class: "cheatsheet-card",
                        h4 { "Plages privées (RFC 1918)" }
                        ul {
                            li { strong { "10.0.0.0/8" } " - 10.0.0.0 à 10.255.255.255" }
                            li { strong { "172.16.0.0/12" } " - 172.16.0.0 à 172.31.255.255" }
                            li { strong { "192.168.0.0/16" } " - 192.168.0.0 à 192.168.255.255" }
                        }
                        h4 { "Adresses spéciales" }
                        ul {
                            li { strong { "127.0.0.0/8" } " - Boucle locale (localhost)" }
                            li { strong { "169.254.0.0/16" } " - APIPA (lien-local)" }
                            li { strong { "224.0.0.0/4" } " - Multicast (classe D)" }
                            li { strong { "0.0.0.0" } " - Adresse indéterminée" }
                            li { strong { "255.255.255.255" } " - Broadcast limité" }
                        }
                    }
                    div { class: "cheatsheet-card",
                        h4 { "Formules essentielles" }
                        ul {
                            li { "Adresse réseau = IP & Masque" }
                            li { "Broadcast = IP | ~Masque" }
                            li { "Wildcard = ~Masque" }
                            li { "Nb hôtes = 2^(32−n) − 2" }
                            li { "Nb sous-réseaux = 2^(n₂−n₁)" }
                            li { "Taille sous-réseau = 2^(32−n₂)" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ToolCard(icon: Asset, title: &'static str, desc: &'static str) -> Element {
    rsx! {
        div { class: "tool-card",
            img { class: "tool-card-icon", src: "{icon}" }
            strong { "{title}" }
            p { "{desc}" }
        }
    }
}
