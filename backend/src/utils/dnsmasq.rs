use std::fs::{read_to_string, remove_file, write};
use std::io::{Result};
use std::path::{Path};
use std::process::exit;
use log::error;

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
}

const CONFIG_CONTENTS: &str = "address=/.local/10.42.0.1";

/// Writes the config file at /etc/NetworkManager/dnsmasq-shared.d/hosts.conf
/// with CONFIG_CONTENTS which indicates which domains to address to this machine
fn write_config_file() -> Result<()> {
    let path = Path::new("/etc/NetworkManager/dnsmasq-shared.d/hosts.conf");
    if path.exists() {
        remove_file(path)?;
    }
    write(path, CONFIG_CONTENTS)?;
    Ok(())
}

const HOSTS_ENTRY: &str = "127.0.0.1 drivepi.local";

/// Writes an entry to the /etc/hosts file which points localhost to
/// drivepi.local which will be used by dnsmasq
fn write_hosts_entry() -> Result<()> {
    let path = Path::new("/etc/hosts");
    let mut contents = read_to_string(path)?;
    if !contents.contains(HOSTS_ENTRY) {
        contents.push('\n');
        contents.push_str(HOSTS_ENTRY);
        contents.push('\n');
        write(path, contents)?;
    }
    Ok(())
}