pub mod app;
pub mod help;
pub mod subnet_split;
pub mod sidebar;
pub mod home;
pub mod ip_range;
pub mod subnet_mask;
pub mod broadcast;
pub mod host_count;


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

pub fn format_ipv4(ip: u32) -> String {
    format!("{}.{}.{}.{}",
        (ip >> 24) & 255,
        (ip >> 16) & 255,
        (ip >> 8) & 255,
        ip & 255
    )
}