use crate::{address::NetAddress, components::{sidebar::SidebarButton, Modal}, HOW_SVG};
use dioxus::prelude::*;
use std::net::Ipv4Addr;
#[allow(non_snake_case)]

#[component]
pub fn WidgetsPage() -> Element {
    let mut current_widget:Signal<Modal> = use_signal(|| Modal::NetAddress);
    let mut show_modal = use_signal(|| false);
    
    rsx! {
        div { class: "tool-container",    
            div{
                SidebarButton {
                    current: current_widget.read().to_owned().to_string(),
                    target: "net-address",
                    onclick: move |_| {current_widget.set(Modal::NetAddress); show_modal.set(true)},
                    svg_path: HOW_SVG,
                    text: "Comment déterminer l'adresse réseau",
                }
            }
            div{
                SidebarButton {
                    current: current_widget.read().to_owned().to_string(),
                    target: "find-ips-addr",
                    onclick: move |_| {current_widget.set(Modal::FindIpsAddr); show_modal.set(true)},
                    svg_path: HOW_SVG,
                    text: "Comment déterminer les adresses utilisables",
                }
            }
            div{
                SidebarButton {
                    current: current_widget.read().to_owned().to_string(),
                    target: "broadcast-addr",
                    onclick: move |_| {current_widget.set(Modal::BroadcastAddr); show_modal.set(true)},
                    svg_path: HOW_SVG,
                    text: "Comment calculer l'adresse de Diffusion",
                }
            }
            div{
                SidebarButton {
                    current: current_widget.read().to_owned().to_string(),
                    target: "subnetting",
                    onclick: move |_| {current_widget.set(Modal::Subnetting); show_modal.set(true)},
                    svg_path: HOW_SVG,
                    text: "Comment Découper un réseau",
                }
            }
            div{
                SidebarButton {
                    current: current_widget.read().to_owned().to_string(),
                    target: "find-mask",
                    onclick: move |_| {current_widget.set(Modal::FindMask); show_modal.set(true)},
                    svg_path: HOW_SVG,
                    text: "Comment calculer le Masque",
                }
            }
        }
        if *show_modal.read() {
        div {
            class: "modal-overlay",
            onclick: move |_| show_modal.set(false), // Ferme si clic sur overlay
            div {
                class: "modal-content",
                onclick: move |evt| evt.stop_propagation(), // ⛔ bloque la propagation
                match current_widget.read().to_owned() {
                    Modal::FindMask => rsx!(FindMask{}),
                    Modal::NetAddress => rsx!(CalculatorNetAddress{}),
                    Modal::FindIpsAddr => rsx!(FindIpsAddr{}),
                    Modal::BroadcastAddr => rsx!(FindBroadcastAddr{}),
                    Modal::Subnetting => rsx!(Subnetting{}),
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

#[component]
pub fn CalculatorNetAddress() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);
    let mut binary_network = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "🧮 Calcul de l'adresse réseau" }
            p { "Saisissez une adresse IP et un masque pour visualiser le calcul binaire (opération AND bit à bit)." }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.10)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        let ip_octets: Vec<u32> = ip_val.octets().iter().map(|b| *b as u32).collect();
                        let ip_u32 = (ip_octets[0] << 24) | (ip_octets[1] << 16) | (ip_octets[2] << 8) | ip_octets[3];
                        let mask_u32 = 0xFFFFFFFFu32 << (32 - mask_val);
                        let network = ip_u32 & mask_u32;

                        result.set(format!("Adresse Réseau : {}.{}.{}.{}",
                            (network >> 24) & 255,
                            (network >> 16) & 255,
                            (network >> 8) & 255,
                            network & 255,
                        ));

                        binary_ip.set(format!("{:032b}", ip_u32));
                        binary_mask.set(format!("{:032b}", mask_u32));
                        binary_network.set(format!("{:032b}", network));
                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_ip.set("".to_string());
                        binary_mask.set("".to_string());
                        binary_network.set("".to_string());
                    }
                },
                "Calculer"
            }

            div {
                style: "margin-top: 1rem;",
                p { "🧠 Représentation binaire (avec couleurs) :" }

                div {
                    style: "font-family: monospace; white-space: pre-wrap;",
                    BitLine { label: "IP".to_string(), bits: binary_ip.read().clone(), color: "cyan".to_string() }
                    BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "orange".to_string() }
                    BitLine { label: "Résultat".to_string(), bits: binary_network.read().clone(), color: "limegreen".to_string() }
                }
            }

            p { class: "result", "{result.read()}" }
        }
    }
}

#[component]
fn BitLine(label: String, bits: String, color: String) -> Element {
    let octets: Vec<String> = bits
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    rsx! {
        div {
            b { "{label} : " }
            for octet in octets {
                span {
                    style: "color: {color}; padding: 0 4px;",
                    "{octet} "
                }
            }
        }
    }
}

#[component]
pub fn FindIpsAddr() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_first_ip = use_signal(String::new);
    let mut binary_last_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "📍 Plage d'adresses utilisables" }
            p { "Saisissez une adresse IP et un masque pour obtenir la première et la dernière adresse IP utilisable du sous-réseau." }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.10)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        let ip_octets: Vec<u32> = ip_val.octets().iter().map(|b| *b as u32).collect();
                        let ip_u32 = (ip_octets[0] << 24) | (ip_octets[1] << 16) | (ip_octets[2] << 8) | ip_octets[3];

                        let (first, last) = NetAddress::ip_range(ip_u32, mask_val);

                        result.set(format!(
                            "Première IP utilisable : {}\nDernière IP utilisable : {}",
                            crate::components::format_ipv4(first),
                            crate::components::format_ipv4(last),
                        ));

                        binary_first_ip.set(format!("{:032b}", first));
                        binary_last_ip.set(format!("{:032b}", last));
                        binary_mask.set(format!("{:032b}", 0xFFFFFFFFu32 << (32 - mask_val)));

                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_first_ip.set("".to_string());
                        binary_last_ip.set("".to_string());
                        binary_mask.set("".to_string());
                    }
                },
                "Calculer"
            }

            p { class: "result", "{result.read()}" }

            div {
                style: "margin-top: 1rem;",
                p { "🧠 Représentation binaire (avec couleurs) :" }

                div {
                    style: "font-family: monospace; white-space: pre-wrap;",
                    BitLine { label: "Première IP".to_string(), bits: binary_first_ip.read().clone(), color: "cyan".to_string() }
                    BitLine { label: "Dernière IP".to_string(), bits: binary_last_ip.read().clone(), color: "orange".to_string() }
                    BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "limegreen".to_string() }
                }
            }
        }
    }
}


#[component]
pub fn FindBroadcastAddr() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_ip = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);
    let mut binary_broadcast = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "📡 Adresse de diffusion" }
            p { "Calcule l'adresse de diffusion à partir d'une IP et d'un masque. (IP OR complément du masque)" }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.10)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>()
                    ) {
                        let ip_u32 = u32::from(ip_val);
                        let broadcast = NetAddress::new(ip_u32, mask_val).broadcast_address();

                        result.set(format!("Adresse de diffusion : {}", crate::components::format_ipv4(broadcast)));

                        binary_ip.set(format!("{:032b}", ip_u32));
                        binary_mask.set(format!("{:032b}", 0xFFFFFFFFu32 << (32 - mask_val)));
                        binary_broadcast.set(format!("{:032b}", broadcast));
                    } else {
                        result.set("Entrées invalides".to_string());
                        binary_ip.set("".to_string());
                        binary_mask.set("".to_string());
                        binary_broadcast.set("".to_string());
                    }
                },
                "Calculer"
            }

            p { class: "result", "{result.read()}" }

             div {
                style: "margin-top: 1rem;",
                p { "🧠 Représentation binaire (avec couleurs) :" }

                div {
                    style: "font-family: monospace; white-space: pre-wrap;",
                    BitLine { label: "IP".to_string(), bits: binary_ip.read().clone(), color: "cyan".to_string() }
                    BitLine { label: "Masque".to_string(), bits: binary_mask.read().clone(), color: "orange".to_string() }
                    BitLine { label: "Diffusion".to_string(), bits: binary_broadcast.read().clone(), color: "limegreen".to_string() }
                }
            }
        }
    }
}


#[component]
pub fn Subnetting() -> Element {
    let mut ip = use_signal(String::new);
    let mut mask = use_signal(String::new);
    let mut new_mask = use_signal(String::new);
    let mut result = use_signal(String::new);

    rsx! {
        div { class: "tool-container",
            h3 { "🔀 Découpage en sous-réseaux" }
            p { "Divise un réseau en plusieurs sous-réseaux à l'aide d'un masque plus précis." }

            input {
                class: "input-field",
                placeholder: "Adresse IP (ex: 192.168.1.0)",
                value: "{ip}",
                oninput: move |e| ip.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Masque actuel (ex: 24)",
                value: "{mask}",
                oninput: move |e| mask.set(e.value().clone())
            }

            input {
                class: "input-field",
                placeholder: "Nouveau masque (ex: 26)",
                value: "{new_mask}",
                oninput: move |e| new_mask.set(e.value().clone())
            }

            button {
                class: "action-button",
                onclick: move |_| {
                    if let (Ok(ip_val), Ok(mask_val), Ok(new_mask_val)) = (
                        ip.read().parse::<Ipv4Addr>(),
                        mask.read().parse::<u32>(),
                        new_mask.read().parse::<u32>()
                    ) {
                        let ip_u32 = u32::from(ip_val);
                        let net = NetAddress::new(ip_u32, mask_val);

                        let sous_reseaux = net.subnet_split(new_mask_val);
                        let lignes = sous_reseaux.iter()
                            .map(|s| format!("{:#}", s)) // Utiliser # pour formater avec le binaire
                            .collect::<Vec<_>>()
                            .join("\n");

                        result.set(lignes);
                    } else {
                        result.set("Entrées invalides".to_string());
                    }
                },
                "Découper"
            }

            pre {
                class: "result",
                "{result.read()}"
            }
        }
    }
}

#[component]
pub fn FindMask() -> Element {
    let mut nb_ips = use_signal(String::new);
    let mut result = use_signal(String::new);
    let mut binary_mask = use_signal(String::new);

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

                        result.set(format!("/{} → {}", mask, crate::components::format_ipv4(full_mask)));
                        binary_mask.set(format!("{:032b}", full_mask));
                    } else {
                        result.set("Entrée invalide".to_string());
                        binary_mask.set("".to_string());
                    }
                },
                "Calculer"
            }

            p { class: "result", "{result.read()}" }

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

