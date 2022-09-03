use std::fs::{read_to_string, remove_file, write};
use std::io::{Result};
use std::path::{Path};
use std::process::{Command, exit};
use log::error;

const HOSTS_FILE_PATH: &str = "/etc/hosts";
const DNSMASQ_CONFIG_PATH: &str = "/etc/NetworkManager/dnsmasq-shared.d/hosts.conf";
const CONFIG_CONTENTS: &str = "address=/.local/10.42.0.1";
const HOSTS_ENTRY: &str = "127.0.0.1 drivepi.local";

/// Sets up dnsmasq to redirect the traffic for drivepi.local to the
/// local machine.
pub fn setup_dnsmasq() {
    write_config_file()
        .unwrap_or_else(|err| {
            error!("Error writing dnsmasq config file: {}", err);
            exit(5)
        });
    write_hosts_entry()
        .unwrap_or_else(|err| {
            error!("Error writing dnsmasq hosts file entry: {}", err);
            exit(6)
        });

    restart_service()
}

/// Writes the config file at /etc/NetworkManager/dnsmasq-shared.d/hosts.conf
/// with CONFIG_CONTENTS which indicates which domains to address to this machine
fn write_config_file() -> Result<()> {
    let path = Path::new(DNSMASQ_CONFIG_PATH);
    if path.exists() {
        remove_file(path)?;
    }
    write(path, CONFIG_CONTENTS)?;
    Ok(())
}

/// Writes an entry to the /etc/hosts file which points localhost to
/// drivepi.local which will be used by dnsmasq
fn write_hosts_entry() -> Result<()> {
    let path = Path::new(HOSTS_FILE_PATH);
    let mut contents = read_to_string(path)?;
    if !contents.contains(HOSTS_ENTRY) {
        contents.push('\n');
        contents.push_str(HOSTS_ENTRY);
        contents.push('\n');
        write(path, contents)?;
    }
    Ok(())
}

fn restart_service() {
    Command::new("systemctl")
        .args(["restart", "dnsmasq"])
        .output()
        .expect("Failed to restart dnsmasq");
}