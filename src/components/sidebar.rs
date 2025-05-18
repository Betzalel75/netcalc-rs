use dioxus::prelude::*;

#[component]
pub fn SidebarButton(
    onclick: EventHandler<MouseEvent>,
    svg_path: Asset,
    text: String,
) -> Element {
    rsx! {
        button {
            class: "sidebar-button",
            onclick: move |evt| onclick.call(evt),
            img { src: "{svg_path}", class: "sidebar-icon" }
            span { "{text}" }
        }
    }
}
