use crate::config::Config;
use std::{collections::HashSet, thread, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};

pub fn start_monitoring(config: &Config) -> anyhow::Result<()> {
    let mut system = System::new_all();
    let mut seen: HashSet<(String, i32)> = HashSet::new(); // (имя процесса, PID)

    let interval = Duration::from_secs(config.check_interval);

    loop {
        system.refresh_processes();

        for process in system.processes().values() {
            let name = process.name().to_ascii_lowercase();
            let pid = process.pid();

            if config.apps.contains(&name) && !seen.contains(&(name.clone(), pid)) {
                println!("Detected target app: {} [PID: {}]", name, pid);
                seen.insert((name.clone(), pid));
                // здесь можно добавить логирование или другую обработку
            }
        }

        thread::sleep(interval);
    }
}
