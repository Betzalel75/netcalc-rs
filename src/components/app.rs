use dioxus::prelude::*;
use crate::components::switcher::Switcher;
#[allow(unused_imports)]
use crate::{components::{broadcast::Broadcast, help::Help, home::Home, host_count::HostCount, ip_range::IpRange, sidebar::SidebarButton, subnet_mask::SubnetMask, subnet_split::SubnetSplit, Theme, View}, BASE_CSS, BROADCAST_SVG, COMPONENT_CSS, DASHBOARD_SVG, FAVICON, HELP_SVG, HOTES_SVG, IPS_SVG, LAYOUT_CSS, MASK_SVG, SUBNET_SVG, VARIABLES_CSS};
#[allow(non_snake_case)]

#[component]
pub fn App() -> Element {
    let mut current_view:Signal<View> = use_signal(|| View::Home);
    let theme = use_signal(|| Theme::System);

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
    /*
    use_effect(move || {
            // Detect initial theme
            theme.set(Theme::System);
    
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::prelude::*;
                use web_sys::*;
    
                let theme_clone = theme.clone();
    
                let callback = Closure::wrap(Box::new(move || {
                    theme_clone.set(Theme::System);
                }) as Box<dyn Fn()>);
    
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
    
                let media_list = window
                    .match_media("(prefers-color-scheme: dark)")
                    .unwrap()
                    .unwrap();
    
                media_list
                    .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
                    .unwrap();
    
                // Prevent the closure from being dropped prematurely
                std::mem::forget(callback);
            }
        });
    */

    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: VARIABLES_CSS}
            document::Link { rel: "stylesheet", href: LAYOUT_CSS}
            document::Link { rel: "stylesheet", href: COMPONENT_CSS}
            document::Link { rel: "stylesheet", href: BASE_CSS}
            body {
            div { class: "glass-container {theme_class}",
            div { class: "dashboard",
                div { class: "sidebar",
                    div {
                        class: "main-content",
                        h2 { "NetCalc-RS" }
                        SidebarButton {
                            current: current_view.read().to_owned().to_string(),
                            target: "home",
                            onclick: move |_| current_view.set(View::Home),
                            svg_path: DASHBOARD_SVG,
                            text: "Home",
                        }
                    },
                    Switcher { theme: theme }
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
                                    text: "Masque depuis nb IP",
                                }
                            }
                            li {
                                SidebarButton {
                                    current: current_view.read().to_owned().to_string(),
                                    target: "broadcast",
                                    onclick: move |_| current_view.set(View::Broadcast),
                                    svg_path: BROADCAST_SVG,
                                    text: "Adresse de diffusion",
                                }
                            }
                            li {
                                SidebarButton {
                                    current: current_view.read().to_owned().to_string(),
                                    target: "host-count",
                                    onclick: move |_| current_view.set(View::HostCount),
                                    svg_path: HOTES_SVG,
                                    text: "Hôtes",
                                }
                            }
                            li {
                                SidebarButton {
                                    current: current_view.read().to_owned().to_string(),
                                    target: "subnet-split",
                                    onclick: move |_| current_view.set(View::SubnetSplit),
                                    svg_path: SUBNET_SVG,
                                    text: "Subdivision",
                                }
                            }
                            li {
                                SidebarButton {
                                    current: current_view.read().to_owned().to_string(),
                                    target: "help",
                                    onclick: move |_| current_view.set(View::Help),
                                    svg_path: HELP_SVG,
                                    text: "Aide",
                                }
                            }
                        }
                    }
                }
                div { class: "main-content",
                    match *current_view.read() {
                        View::Home => rsx!(Home {}),
                        View::IpRange => rsx!(IpRange {}),
                        View::SubnetMask => rsx!(SubnetMask {}),
                        View::Broadcast => rsx!(Broadcast {}),
                        View::HostCount => rsx!(HostCount {}),
                        View::SubnetSplit => rsx!(SubnetSplit {}),
                        View::Help => rsx!(Help{}),
                    }
                }
            }
            }
            }
    }
}
