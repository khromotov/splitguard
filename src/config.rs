pub struct Config {
    pub wg_config_path: String,
    pub tun2socks_path: String,
    pub socks5_port: u16,
    pub check_interval: u64,
    pub app_list: Vec<String>,
}
