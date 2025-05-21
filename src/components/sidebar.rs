use dioxus::prelude::*;
#[allow(non_snake_case)]

#[component]
pub fn SidebarButton(
    onclick: EventHandler<MouseEvent>,
    svg_path: Asset,
    text: String,
    current: &'static str,
    target: &'static str,
) -> Element {
    let is_active = current == target;
    rsx! {
        button {
            class: "sidebar-button {is_active.then_some(\"active\").unwrap_or_default()}",
            onclick: move |evt| onclick.call(evt),
            img { src: "{svg_path}", class: "sidebar-icon" }
            span { "{text}" }
        }
    }
}
