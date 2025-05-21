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
pub mod widget;


#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Home,
    IpRange,
    SubnetMask,
    Broadcast,
    HostCount,
    SubnetSplit,
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


pub fn format_ipv4(ip: u32) -> String {
    format!("{}.{}.{}.{}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}

impl View {
    pub fn to_string(&self) -> &'static str {
        match self {
            View::Home => "home",
            View::IpRange => "ip-range",
            View::SubnetMask => "subnet-mask",
            View::Broadcast => "broadcast",
            View::HostCount => "host-count",
            View::SubnetSplit => "subnet-split",
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
