use crate::config::Config;
use std::process::{Command, Stdio};

pub fn start(config: &Config) -> anyhow::Result<()> {
    Command::new(&config.tun2socks_path)
        .args([
            "--tundev",
            "wg0",
            "--netif-ipaddr",
            "10.0.0.2",
            "--socks-server-addr",
            &format!("127.0.0.1:{}", config.socks5_port.to_string()),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
