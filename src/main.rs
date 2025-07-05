mod config;
mod monitor;
mod tun2socks;
mod wg;

use config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::load("config.toml")?;

    println!("Starting WireGuard...");
    wg::start(&config.wg_config_path)?;

    println!("Starting tun2socks...");
    tun2socks::start(&config)?;

    println!("Monitoring apps...");
    monitor::start_monitoring(&config)?;

    Ok(())
}
