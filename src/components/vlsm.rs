use dioxus::prelude::*;

use crate::address::{calcmask, format_ipv4, NetAddress};

#[derive(Clone, PartialEq)]
struct VlsmSubnet {
    name: String,
    hosts: u32,
    mask: u32,
    network: u32,
    first: u32,
    last: u32,
}

#[component]
pub fn Vlsm() -> Element {
    let mut network_ip = use_signal(String::new);
    let mut network_mask = use_signal(String::new);
    let mut subnet_input = use_signal(String::new);
    let mut result = use_signal(|| Vec::<VlsmSubnet>::new());
    let mut error = use_signal(String::new);
    let mut is_valid = use_signal(|| true);

    let input_class = move || {
        if *is_valid.read() { "input-field" } else { "input-field invalid" }
    };

    rsx! {
        div { class: "tool-container",
            h3 { "VLSM : Masque de sous-réseau à longueur variable" }
            p { "Découpe un réseau en sous-réseaux de tailles variables." }
            p { class: "hint", "Entrez un sous-réseau par ligne au format : nom,hôtes (ex: RH,30)" }

            input { class: "{input_class()}", placeholder: "Adresse réseau (ex: 192.168.1.0)", value: "{network_ip}",
                oninput: move |e| { network_ip.set(e.value().clone()); is_valid.set(true); } }
            input { class: "{input_class()}", placeholder: "Masque (ex: 24)", value: "{network_mask}",
                oninput: move |e| { network_mask.set(e.value().clone()); is_valid.set(true); } }

            textarea {
                class: "input-field",
                placeholder: "Sous-réseaux :\nRH,30\nIT,50\nDMZ,10",
                rows: "4",
                value: "{subnet_input}",
                oninput: move |e| { subnet_input.set(e.value().clone()); is_valid.set(true); }
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    let ip_result = network_ip.read().parse::<std::net::Ipv4Addr>();
                    let mask_result = network_mask.read().parse::<u32>();
                    if let (Ok(ip), Ok(mask)) = (ip_result, mask_result) {
                        match NetAddress::from_ip_and_mask(ip, mask) {
                            Ok(base_net) => {
                                let mut requirements: Vec<(String, u32)> = Vec::new();
                                for line in subnet_input.read().lines() {
                                    let line = line.trim();
                                    if line.is_empty() { continue; }
                                    let parts: Vec<&str> = line.split(',').collect();
                                    if parts.len() == 2 {
                                        if let Ok(hosts) = parts[1].trim().parse::<u32>() {
                                            if hosts > 0 {
                                                requirements.push((parts[0].trim().to_string(), hosts));
                                            }
                                        }
                                    }
                                }
                                if requirements.is_empty() {
                                    is_valid.set(false);
                                    error.set("Aucun sous-réseau valide spécifié.".to_string());
                                    result.set(Vec::new());
                                    return;
                                }
                                requirements.sort_by(|a, b| b.1.cmp(&a.1));

                                let mut subnets: Vec<VlsmSubnet> = Vec::new();
                                let mut current_ip = base_net.network_address();
                                let max_ip = base_net.broadcast_address();
                                let mut ok = true;

                                for (name, hosts) in &requirements {
                                    let needed_mask = match calcmask(*hosts) {
                                        Ok(m) => m,
                                        Err(_) => { ok = false; break; }
                                    };
                                    let new_net = NetAddress::new(current_ip, needed_mask);
                                    let subnet_size = 2u32.pow(32 - needed_mask);
                                    if new_net.broadcast_address() > max_ip {
                                        error.set(format!("Espace insuffisant pour '{}'.", name));
                                        is_valid.set(false);
                                        result.set(Vec::new());
                                        ok = false;
                                        break;
                                    }
                                    let (first, last) = new_net.ip_range();
                                    subnets.push(VlsmSubnet {
                                        name: name.clone(), hosts: *hosts, mask: needed_mask,
                                        network: new_net.network_address(), first, last,
                                    });
                                    current_ip = current_ip.wrapping_add(subnet_size);
                                }

                                if ok {
                                    is_valid.set(true);
                                    error.set("".to_string());
                                    result.set(subnets);
                                }
                            }
                            Err(e) => { is_valid.set(false); error.set(e.to_string()); result.set(Vec::new()); }
                        }
                    } else {
                        is_valid.set(false);
                        error.set("Entrées réseau invalides.".to_string());
                        result.set(Vec::new());
                    }
                },
                "Calculer VLSM"
            }

            if !error.read().is_empty() {
                p { class: "result error", "{error.read()}" }
            }
            if !result.read().is_empty() {
                p { class: "result info", "{result.read().len()} sous-réseaux VLSM générés" }
                {build_vlsm_table(&result.read())}
            }
        }
    }
}

fn build_vlsm_table(subnets: &[VlsmSubnet]) -> Element {
    let mut table = crate::display_table::Table::new();
    table.headers = vec!["Segment".to_string(), "Sous-réseau".to_string(), "Masque".to_string(), "Hôtes".to_string(), "Plage".to_string()];
    for s in subnets {
        table.add_row(&[
            s.name.clone(),
            format!("{}/{}", format_ipv4(s.network), s.mask),
            format!("/{}", s.mask),
            s.hosts.to_string(),
            format!("{} – {}", format_ipv4(s.first), format_ipv4(s.last)),
        ]);
    }
    rsx! { crate::components::widgets::utils::Tables { table } }
}
