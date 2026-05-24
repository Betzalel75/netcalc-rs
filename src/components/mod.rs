pub mod app;
pub mod help;
pub mod subnet_split;
pub mod sidebar;
pub mod home;
pub mod ip_range;
pub mod subnet_mask;
pub mod broadcast;
pub mod host_count;
pub mod switcher;
pub mod widgets;

// Nouvelles fonctionnalités
pub mod vlsm;
pub mod summarize;
pub mod ip_checker;
pub mod wildcard;
pub mod converter;

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Home,
    IpRange,
    SubnetMask,
    Broadcast,
    HostCount,
    SubnetSplit,
    Vlsm,
    Summarize,
    IpChecker,
    Wildcard,
    Converter,
    Help,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Modal {
    NetAddress,
    FindIpsAddr,
    BroadcastAddr,
    Subnetting,
    FindMask,
}


pub use crate::address::format_ipv4;

impl View {
    pub fn to_string(&self) -> &'static str {
        match self {
            View::Home => "home",
            View::IpRange => "ip-range",
            View::SubnetMask => "subnet-mask",
            View::Broadcast => "broadcast",
            View::HostCount => "host-count",
            View::SubnetSplit => "subnet-split",
            View::Vlsm => "vlsm",
            View::Summarize => "summarize",
            View::IpChecker => "ip-checker",
            View::Wildcard => "wildcard",
            View::Converter => "converter",
            View::Help => "help",
        }
    }
}

impl Theme {
    pub fn to_string(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}

impl Modal {
    pub fn to_string(&self) -> &'static str {
        match self {
            Modal::NetAddress => "net-address",
            Modal::FindIpsAddr => "find-ips-addr",
            Modal::BroadcastAddr => "broadcast-addr",
            Modal::Subnetting => "subnetting",
            Modal::FindMask => "find-mask",
        }
    }
}
