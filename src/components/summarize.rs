use dioxus::prelude::*;

use crate::address::{format_ipv4, NetAddress, summarize};

#[component]
pub fn Summarize() -> Element {
    let mut routes_input = use_signal(String::new);
    let mut result = use_signal(|| None::<NetAddress>);
    let mut routes_detail = use_signal(|| Vec::<NetAddress>::new());
    let mut error = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    rsx! {
        div { class: "tool-container",
            h3 { "Agrégation de routes (supernetting)" }
            p { "Regroupe plusieurs sous-réseaux contigus en un super-réseau unique." }
            p { class: "hint", "Entrez une route par ligne au format CIDR (ex: 192.168.1.0/24)" }

            textarea {
                class: if *is_valid.read() { "input-field" } else { "input-field invalid" },
                placeholder: "Routes :\n192.168.1.0/24\n192.168.2.0/24\n192.168.3.0/24",
                rows: "5",
                value: "{routes_input}",
                oninput: move |e| { routes_input.set(e.value().clone()); is_valid.set(true); }
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    let mut routes = Vec::new();
                    for line in routes_input.read().lines() {
                        let line = line.trim();
                        if line.is_empty() { continue; }
                        match NetAddress::from_str(line) {
                            Ok(net) => routes.push(net),
                            Err(e) => {
                                is_valid.set(false);
                                error.set(format!("Erreur sur '{}': {}", line, e));
                                result.set(None);
                                routes_detail.set(Vec::new());
                                return;
                            }
                        }
                    }
                    if routes.is_empty() {
                        is_valid.set(false);
                        error.set("Aucune route valide.".to_string());
                        result.set(None);
                        routes_detail.set(Vec::new());
                        return;
                    }
                    match summarize(&routes) {
                        Ok(summary) => {
                            is_valid.set(true); error.set("".to_string());
                            result.set(Some(summary)); routes_detail.set(routes);
                        }
                        Err(e) => {
                            is_valid.set(false); error.set(e.to_string());
                            result.set(None); routes_detail.set(Vec::new());
                        }
                    }
                },
                "Agréger"
            }

            if !error.read().is_empty() {
                p { class: "result error", "{error.read()}" }
            }

            if let Some(summary) = *result.read() {
                div { class: "result",
                    h4 { "Résultat de l'agrégation" }
                    p { "Super-réseau : " b { "{summary.to_cidr_string()}" } }
                    p { "Masque décimal : " b { "{format_ipv4(summary.subnet_mask())}" } }
                    p { "Adresses totales : " b { "{summary.total_addresses()}" } }
                    p { "Hôtes disponibles : " b { "{summary.host_count()}" } }
                }

                if !routes_detail.read().is_empty() {
                    p { class: "result info", "Routes agrégées : {routes_detail.read().len()}" }
                    {build_summary_table(&routes_detail.read())}
                }
            }
        }
    }
}

fn build_summary_table(routes: &[NetAddress]) -> Element {
    let mut table = crate::display_table::Table::new();
    table.headers = vec!["Route".to_string(), "Réseau".to_string(), "Broadcast".to_string()];
    for r in routes {
        table.add_row(&[
            r.to_cidr_string(),
            format_ipv4(r.network_address()),
            format_ipv4(r.broadcast_address()),
        ]);
    }
    rsx! { crate::components::widgets::utils::Tables { table } }
}
