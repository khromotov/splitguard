fn main() -> anyhow::Result<()> {
    let config = config::load_config("config.toml")?;
    wg::start(&config.wg_config_path)?;
    tun2socks::start(&config)?;
    monitor::start_monitoring(&config)?;
    Ok(())
}
