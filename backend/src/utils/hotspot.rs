use std::env;
use std::process::{Command, exit};
use log::{error, info};

// Default setting constants
const DEFAULT_HOTSPOT_INTERFACE: &str = "wlan0";
const DEFAULT_HOTSPOT_SSID: &str = "Drive-PI";
const DEFAULT_HOTSPOT_PASSWORD: &str = "Drive-PI";

/// Starts the hotspot using the system command
pub fn start_hotspot() {

    // Settings loaded through environment variables with defaults
    let interface = env::var("DRIVEPI_HOTSPOT_INTERFACE")
        .unwrap_or_else(|_| String::from(DEFAULT_HOTSPOT_INTERFACE));
    let ssid = env::var("DRIVEPI_HOTSPOT_SSID")
        .unwrap_or(String::from(DEFAULT_HOTSPOT_SSID));
    let password = env::var("DRIVEPI_HOTSPOT_PASSWORD")
        .unwrap_or(String::from(DEFAULT_HOTSPOT_PASSWORD));

    // Ensure the password is long enough
    if password.len() < 8 {
        error!("The password provided for the hotspot by the DRIVEPI_HOTSPOT_PASSWORD environment");
        error!("variable is too short. Minimum length required is 8 characters.");
        exit(1);
    }

    // Create the hotspot using the nmcli (network-manager cli) tool
    let output = Command::new("nmcli")
        .args([
            "d", "wifi", "hotspot",
            "ifname", interface,
            "ssid", ssid.clone(),
            "password", password.clone()
        ])
        .output()
        .unwrap_or_else(|_| {
            error!("Failed to start hotspot: {}", err);
            exit(2);
        });

    // Parse the stdout as a string
    let output = String::from_utf8_lossy(&output.stdout)
        .as_ref();

    // Fail if the message doesn't say success
    if !output.contains("successfully activated") {
        error!("Failed to start hotspot: {}", output);
        exit(3);
    }

    info!("Created hotspot named {} with password {}", ssid, password);
}

