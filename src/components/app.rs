use dioxus::prelude::*;
use crate::components::switcher::Switcher;
use crate::{
    components::{
        broadcast::Broadcast, converter::Converter, help::Help, home::Home,
        host_count::HostCount, ip_checker::IpChecker, ip_range::IpRange,
        sidebar::SidebarButton, subnet_mask::SubnetMask, subnet_split::SubnetSplit,
        summarize::Summarize, vlsm::Vlsm, wildcard::Wildcard, Theme, View,
    },
    BASE_CSS, BROADCAST_SVG, COMPONENT_CSS, DASHBOARD_SVG, FAVICON, HELP_SVG, HOTES_SVG,
    HOW_SVG, IPS_SVG, LAYOUT_CSS, MASK_SVG, SUBNET_SVG, VARIABLES_CSS,
};

/// Historique partagé entre les composants.
pub type History = Signal<Vec<String>>;

#[component]
pub fn App() -> Element {
    let mut current_view: Signal<View> = use_signal(|| View::Home);
    let theme = use_signal(|| Theme::System);
    let history: History = use_signal(Vec::new);

    let theme_class: &'static str = match *theme.read() {
        Theme::Light => "light",
        Theme::Dark => "dark",
        Theme::System => {
            if dark_light::detect().unwrap() == dark_light::Mode::Dark {
                "dark"
            } else {
                "light"
            }
        }
    };

    // Fournir l'historique dans le contexte
    use_context_provider(|| history);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: VARIABLES_CSS }
        document::Link { rel: "stylesheet", href: LAYOUT_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: BASE_CSS }
        body {
            div { class: "glass-container {theme_class}",
                div { class: "dashboard",
                    // ── Sidebar ──────────────────────────────────────
                    div { class: "sidebar",
                        div { class: "main-content",
                            h2 { "NetCalc-RS" }
                            SidebarButton {
                                current: current_view.read().to_owned().to_string(),
                                target: "home",
                                onclick: move |_| current_view.set(View::Home),
                                svg_path: DASHBOARD_SVG,
                                text: "Tableau de bord",
                            }
                        }
                        Switcher { theme }

                        // ── Calculs de base ──
                        nav {
                            ul {
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "ip-range",
                                        onclick: move |_| current_view.set(View::IpRange),
                                        svg_path: IPS_SVG,
                                        text: "Plage d'IP",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "subnet-mask",
                                        onclick: move |_| current_view.set(View::SubnetMask),
                                        svg_path: MASK_SVG,
                                        text: "Masque → hôtes",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "broadcast",
                                        onclick: move |_| current_view.set(View::Broadcast),
                                        svg_path: BROADCAST_SVG,
                                        text: "Broadcast",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "host-count",
                                        onclick: move |_| current_view.set(View::HostCount),
                                        svg_path: HOTES_SVG,
                                        text: "Nb d'hôtes",
                                    }
                                }
                            }
                        }

                        hr {}

                        // ── Outils avancés ──
                        nav {
                            ul {
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "subnet-split",
                                        onclick: move |_| current_view.set(View::SubnetSplit),
                                        svg_path: SUBNET_SVG,
                                        text: "Subdivision (FLSM)",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "vlsm",
                                        onclick: move |_| current_view.set(View::Vlsm),
                                        svg_path: SUBNET_SVG,
                                        text: "VLSM",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "summarize",
                                        onclick: move |_| current_view.set(View::Summarize),
                                        svg_path: SUBNET_SVG,
                                        text: "Agrégation",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "ip-checker",
                                        onclick: move |_| current_view.set(View::IpChecker),
                                        svg_path: HELP_SVG,
                                        text: "IP ∈ sous-réseau ?",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "wildcard",
                                        onclick: move |_| current_view.set(View::Wildcard),
                                        svg_path: MASK_SVG,
                                        text: "Wildcard Mask",
                                    }
                                }
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "converter",
                                        onclick: move |_| current_view.set(View::Converter),
                                        svg_path: HOW_SVG,
                                        text: "Convertisseur",
                                    }
                                }
                            }
                        }

                        hr {}

                        // ── Aide ──
                        nav {
                            ul {
                                li {
                                    SidebarButton {
                                        current: current_view.read().to_owned().to_string(),
                                        target: "help",
                                        onclick: move |_| current_view.set(View::Help),
                                        svg_path: HELP_SVG,
                                        text: "Aide & Référence",
                                    }
                                }
                            }
                        }

                        // ── Historique ──
                        HistoryPanel {}
                    }

                    // ── Contenu principal ───────────────────────────
                    div { class: "main-content",
                        match *current_view.read() {
                            View::Home => rsx!(Home {}),
                            View::IpRange => rsx!(IpRange {}),
                            View::SubnetMask => rsx!(SubnetMask {}),
                            View::Broadcast => rsx!(Broadcast {}),
                            View::HostCount => rsx!(HostCount {}),
                            View::SubnetSplit => rsx!(SubnetSplit {}),
                            View::Vlsm => rsx!(Vlsm {}),
                            View::Summarize => rsx!(Summarize {}),
                            View::IpChecker => rsx!(IpChecker {}),
                            View::Wildcard => rsx!(Wildcard {}),
                            View::Converter => rsx!(Converter {}),
                            View::Help => rsx!(Help {}),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HistoryPanel() -> Element {
    let history = use_context::<History>();

    // Lire les 5 derniers, dans l'ordre inverse (plus récent en haut)
    let items: Vec<String> = history
        .read()
        .iter()
        .rev()
        .take(5)
        .cloned()
        .collect();

    if items.is_empty() {
        return rsx! {
            div { class: "history-panel",
                p { class: "hint", "Historique vide" }
            }
        };
    }

    rsx! {
        div { class: "history-panel",
            h4 { "📜 Historique" }
            ul {
                for item in items {
                    li { "{item}" }
                }
            }
        }
    }
}
