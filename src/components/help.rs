use dioxus::prelude::*;

use crate::components::widget::WidgetsPage;
#[allow(non_snake_case)]


#[component]
pub fn Help() -> Element {
    rsx! {
        div { class: "help-container",
            h3 { "Aide" }
            p { "Plage d'IP: Déterminer l'adresse IP, la première et la dernière adresse IP d'un sous-réseau" }
            p { "Masque depuis nb IP: Déterminer le masque de sous-réseau qui peut prendre en charge un nombre d'adresses IP" }
            p { "Adresse de diffusion: Déterminer l'adresse de diffusion d'un réseau" }
            p { "Hôtes: Déterminer le nombre d'adresses d'hôtes disponibles sur un réseau" }
            p { "Subdivision: Découpe du sous-réseau" }
        }
        WidgetsPage{}
    }
}
