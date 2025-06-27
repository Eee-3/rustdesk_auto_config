use log::{debug, error, info};
use runas::Command;
use serde::Deserialize;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use toml_edit::{DocumentMut, value};

#[derive(Debug, Deserialize)]
struct Config {
    installer_filename: String,
    // server_address: String,
    server_host: String,
    key: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;
    info!("Loaded config: {:?}", config);
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    // 加载配置文件
    let config = load_config()?;

    install_rustdesk(&config)?;
    let user_profile = std::env::var("USERPROFILE")?;
    let config_folder = PathBuf::from(format!(
        "{user_profile}\\AppData\\Roaming\\RustDesk\\config"
    ));
    info!("The config folder: {}", config_folder.display());
    let config_file = config_folder.join("RustDesk2.toml");
    info!("The config file: {}", config_file.display());

    info!("Killing Rustdesk...");
    let status = Command::new("powershell")
        .arg("-c").arg("kill -Force -Name rustdesk")
        .show(false)
        .status()?;
    if !status.success() {
        error!("Failed to kill Rustdesk: {}", status);
        return Err("Failed to kill Rustdesk".into());
    }

    info!("Start Writing Config...");
    let content: String = std::fs::read_to_string(config_file.clone())?;
    debug!("content: {}", content);
    let mut doc = content.parse::<DocumentMut>()?;
    debug!("{:#?}", doc);
    // doc["rendezvous_server"] = value(&config.server_address);
    doc["options"]["custom-rendezvous-server"] = value(&config.server_host);
    doc["options"]["stop-service"] = value("Y");
    doc["options"]["key"] = value(&config.key);
    std::fs::write(config_file, doc.to_string())?;
    info!("Done writing config");

    info!("Restarting Service...");
    install_service()?;

    Ok(())
    // println!("Hello, world from main-2!");
}

fn install_service() -> Result<(), Box<dyn std::error::Error>> {
    let programs64 = std::env::var("ProgramW6432")?;
    let rustdesk_exe_path = PathBuf::from(format!("{programs64}\\RustDesk\\rustdesk.exe"));
    info!("The rustdesk exe path: {}", rustdesk_exe_path.display());
    let status = Command::new(rustdesk_exe_path)
        .arg("--install-service")
        .show(false)
        .status()?;
    if !status.success() {
        error!("Failed to Install Service: {}", status);
        Err("Failed to Install Service".into())
    } else {
        Ok(())
    }
}

fn install_rustdesk(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Installing Rustdesk from: {}", config.installer_filename);
    let current_dir = std::env::current_dir()?;
    info!("Current dir: {}", current_dir.display());
    // let install_command = format!(r#"cd /d "{}" && "{}" --silent-install"#, current_dir.display(),config.installer_filename);
    let install_command = format!(r#"cd '{}' ; &'{}' --silent-install"#, current_dir.display(),config.installer_filename);
    debug!("install_command: {}", install_command);
    let status = Command::new("powershell")
        .arg("-c")
        .arg(&install_command)
        .show(true)
        .status()?;
    if !status.success() {
        error!("Failed to install Rustdesk: {}", status);
        return Err("Failed to install Rustdesk".into());
    }
    info!("Waiting for 10 seconds for Rustdesk to install...");
    for i in (1..=10).rev() {
        info!("{}", i);
        sleep(Duration::from_secs(1));
    }
    // sleep(Duration::from_secs(10));
    info!("Installing Rustdesk Service...");
    install_service()?;
    info!("Waiting for 10 seconds for Rustdesk Service to install...");
    for i in (1..=10).rev() {
        info!("{}", i);
        sleep(Duration::from_secs(1));
    }
    // sleep(Duration::from_secs(10));
    Ok(())
}
