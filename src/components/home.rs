use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Bienvenue dans NetCalc-RS !" }
        p { "Sélectionnez une option dans le menu à gauche." }
    }
}
