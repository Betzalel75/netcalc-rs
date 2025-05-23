use calculator_net_address::CalculatorNetAddress;
use dioxus::prelude::*;
use find_broadcast_addr::FindBroadcastAddr;
use find_ips_addr::FindIpsAddr;
use find_mask::FindMask;
use subnetting::Subnetting;

use crate::{components::{sidebar::SidebarButton, Modal}, HOW_SVG};
pub mod find_broadcast_addr;
pub mod find_ips_addr;
pub mod find_mask;
pub mod calculator_net_address;
pub mod subnetting;
pub mod utils;



#[allow(non_snake_case)]
#[component]
pub fn WidgetsPage() -> Element {
    let mut current_widget: Signal<Modal> = use_signal(|| Modal::NetAddress);
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
                    class: "action-button close",
                    onclick: move |_| show_modal.set(false),
                    "Fermer"
                }
            }
        }
        }
    }
}
