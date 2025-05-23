use dioxus::prelude::*;
use crate::display_table::Table;
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
