use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub wg_config_path: String,
    pub tun2socks_path: String,
    pub socks5_port: u16,
    pub check_interval: u64,
    pub app_list: String,
    #[serde(skip)]
    pub apps: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let raw = fs::read_to_string(path)?;
        let mut cfg: Config = toml::from_str(&raw)?;

        // Read apps.txt
        let app_raw = fs::read_to_string(&cfg.app_list)?;
        cfg.apps = app_raw
            .lines()
            .map(|l| l.trim().to_ascii_lowercase())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();

        Ok(cfg)
    }
}
