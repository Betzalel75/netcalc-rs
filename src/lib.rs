pub use dioxus::prelude::*;
pub mod address;
pub mod components;
pub mod display_table;

pub const FAVICON: Asset = asset!("/assets/netcalc-rs.ico");
pub const VARIABLES_CSS: Asset = asset!("/assets/css/variables.css");
pub const BASE_CSS: Asset = asset!("/assets/css/base.css");
pub const LAYOUT_CSS: Asset = asset!("/assets/css/layout.css");
pub const COMPONENT_CSS: Asset = asset!("/assets/css/components.css");

pub const BROADCAST_SVG: Asset = asset!("/assets/images/broadcast.svg");
pub const DASHBOARD_SVG: Asset = asset!("/assets/images/dashboard.svg");
pub const HOTES_SVG: Asset = asset!("/assets/images/hotes.svg");
pub const IPS_SVG: Asset = asset!("/assets/images/ips.svg");
pub const MASK_SVG: Asset = asset!("/assets/images/mask.svg");
pub const SUBNET_SVG: Asset = asset!("/assets/images/subnet.svg");
pub const HELP_SVG: Asset = asset!("/assets/images/help.svg");
pub const HOW_SVG: Asset = asset!("/assets/images/person-reading-a-book-question-mark.svg");

pub const SCRIPT: Asset = asset!("/assets/js/index.js");