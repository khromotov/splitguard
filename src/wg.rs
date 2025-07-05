use std::process::Command;

pub fn start(config_path: &str) -> anyhow::Result<()> {
    Command::new("wg-quick")
        .arg("up")
        .arg(config_path)
        .spawn()?;
    Ok(())
}
