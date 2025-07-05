pub fn start_monitoring(config: &Config) -> anyhow::Result<()> {
    loop {
        let running = get_running_processes(); // с помощью sysinfo
        for proc in running {
            if config.app_list.contains(&proc.name) {
                // Логика: пометить, направить через VPN, и т.п.
            }
        }
        std::thread::sleep(Duration::from_secs(config.check_interval));
    }
}
