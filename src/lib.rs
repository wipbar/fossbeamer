use std::net::IpAddr;

pub mod browser;
pub mod display;

/// Returns all IP addresses on all interfaces.
pub fn get_ips() -> Vec<IpAddr> {
    let all_interfaces = pnet::datalink::interfaces();
    all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .map(|i| i.ips)
        .flatten()
        .map(|i| i.ip())
        .collect()
}
