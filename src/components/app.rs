use dioxus::prelude::*;
use crate::{components::{broadcast::Broadcast, help::Help, home::Home, host_count::HostCount, ip_range::IpRange, sidebar::SidebarButton, subnet_mask::SubnetMask, subnet_split::SubnetSplit, Theme, View}, BROADCAST_SVG, DASHBOARD_SVG, FAVICON, HELP_SVG, HOTES_SVG, IPS_SVG, MAIN_CSS, MASK_SVG, SUBNET_SVG};

#[component]
pub fn App() -> Element {
    let mut current_view = use_signal(|| View::Home);
    let mut theme = use_signal(|| Theme::System);

    let theme_class = match *theme.read() {
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

    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS}
            div { class: "{theme_class}",
            div { class: "dashboard",
                div { class: "sidebar",
                    div {
                        h2 { "NetCalc-RS" }
                        SidebarButton {
                            onclick: move |_| current_view.set(View::Home),
                            svg_path: DASHBOARD_SVG,
                            text: "Home",
                        }
                    },
                    div { class: "theme-switcher",
                        label {
                            input {
                                r#type: "radio",
                                name: "theme",
                                value: "system",
                                checked: *theme.read() == Theme::System,
                                onclick: move |_| theme.set(Theme::System),
                            }
                            "System"
                        }
                        label {
                            input {
                                r#type: "radio",
                                name: "theme",
                                value: "light",
                                checked: *theme.read() == Theme::Light,
                                onclick: move |_| theme.set(Theme::Light),
                            }
                            "Light"
                        }
                        label {
                            input {
                                r#type: "radio",
                                name: "theme",
                                value: "dark",
                                checked: *theme.read() == Theme::Dark,
                                onclick: move |_| theme.set(Theme::Dark),
                            }
                            "Dark"
                        }
                    }
                    nav {
                        ul {
                            li {
                                SidebarButton {
                                    onclick: move |_| current_view.set(View::IpRange),
                                    svg_path: IPS_SVG,
                                    text: "Plage d'IP",
                                }
                            }
                            li {
                                SidebarButton {
                                    onclick: move |_| current_view.set(View::SubnetMask),
                                    svg_path: MASK_SVG,
                                    text: "Masque depuis nb IP",
                                }
                            }
                            li {
                                SidebarButton {
                                    onclick: move |_| current_view.set(View::Broadcast),
                                    svg_path: BROADCAST_SVG,
                                    text: "Adresse de diffusion",
                                }
                            }
                            li {
                                SidebarButton {
                                    onclick: move |_| current_view.set(View::HostCount),
                                    svg_path: HOTES_SVG,
                                    text: "Hôtes",
                                }
                            }
                            li {
                                SidebarButton {
                                    onclick: move |_| current_view.set(View::SubnetSplit),
                                    svg_path: SUBNET_SVG,
                                    text: "Subdivision",
                                }
                            }
                            li {
                                SidebarButton {
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
