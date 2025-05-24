use crate::display_table::Table;
use dioxus::prelude::*;
#[allow(non_snake_case)]
#[component]
pub fn Tables(table: Table) -> Element {
    rsx! {
        table {
            thead {
                tr {
                    for col in table.headers{
                        th { "{col}" }
                    }
                }
            }
            tbody {
                for rows in table.body {
                tr {
                    for row in rows{
                        td { "{row}" }
                    }
                }
                }
            }
        }
    }
}

#[component]
pub fn BitLine(label: String, bits: String, color: String) -> Element {
    let octets: Vec<String> = bits
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    rsx! {
        div {
            style: "display: flex; align-items: center; margin-bottom: 4px;",

            span {
                style: "display: inline-block; width: 80px; text-align: left; margin-left: 10px;",
                b { "{label} :" }
            }

            div {
                style: "font-family: monospace;",
                for octet in octets {
                    span {
                        style: "color: {color}; margin-left: 8px;",
                        "{octet}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn BitCell(bit: char) -> Element {

    rsx! {
        div {
            class: "bit",
            "{bit}"
        }
    }
}

#[component]
pub fn BitLines(
    label: String,
    bits: String,
    line: u16,
    part: usize
) -> Element {
    let start_index = (part - 1) * 8;
    let end_index = start_index + 8;
    let bits_clone = &bits[start_index..end_index];
    rsx! {
        div {
            if line == 2 {
                b { "{label} ", "&" }
            }
            if line == 3 {
                hr {}
                b { "{label} ", "=" }
            }
            div {
                style: "font-family: monospace; display:flex;",
                class: "calcul",
                div { class: "ligne", id: "ligne{line}",
                    span { class: "etiquette" }
                for bit in bits_clone.chars() {
                    BitCell { bit: bit }
                }
                }
            }
        }
    }
}
