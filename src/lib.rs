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

pub fn setup_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env()
                .expect("Invalid RUST_LOG"),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stderr)
                .compact(),
        );

    subscriber.try_init().expect("failed to setup tracing");
}
