use dioxus::prelude::*;
use crate::components::Theme;
#[allow(non_snake_case)]


#[component]
pub fn Switcher(theme: Signal<Theme>) -> Element {
    rsx! {
        div { class: "theme-switcher",
            ThemeButton {
                current_theme: *theme.read(),
                target_theme: Theme::System,
                on_click: move |_| theme.set(Theme::System),
                label: "🌓 System",
            }
            ThemeButton {
                current_theme: *theme.read(),
                target_theme: Theme::Light,
                on_click: move |_| theme.set(Theme::Light),
                label: "☀️ Light",
            }
            ThemeButton {
                current_theme: *theme.read(),
                target_theme: Theme::Dark,
                on_click: move |_| theme.set(Theme::Dark),
                label: "🌙 Dark",
            }
        }
    }
}

#[component]
fn ThemeButton(
    current_theme: Theme,
    target_theme: Theme,
    on_click: EventHandler<MouseEvent>,
    label: &'static str,
) -> Element {
    let is_active = current_theme == target_theme;

    rsx! {
        button {
            class: "theme-btn {is_active.then_some(\"active\").unwrap_or_default()}",
            onclick: move |evt| on_click.call(evt),
            "{label}"
        }
    }
}