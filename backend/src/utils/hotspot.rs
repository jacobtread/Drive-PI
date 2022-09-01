use std::env;
use std::process::{Command, exit};
use log::{error, info};

// Default setting constants
const DEFAULT_HOTSPOT_INTERFACE: &str = "wlan0";
const DEFAULT_HOTSPOT_SSID: &str = "Drive-PI";
const DEFAULT_HOTSPOT_PASSWORD: &str = "Drive-PI";

// Environment variable keys
const ENV_HOTSPOT_INTERFACE: &str = "DRIVEPI_HOTSPOT_INTERFACE";
const ENV_HOTSPOT_SSID: &str = "DRIVEPI_HOTSPOT_SSID";
const ENV_HOTSPOT_PASSWORD: &str = "DRIVEPI_HOTSPOT_PASSWORD";

const HOTSPOT_CONN_NAME: &str = "Hotspot";

pub struct Hotspot;

impl Hotspot {
    pub fn start() -> Self {
        // Settings loaded through environment variables with defaults
        let interface = env::var(ENV_HOTSPOT_INTERFACE)
            .unwrap_or_else(|_| String::from(DEFAULT_HOTSPOT_INTERFACE));
        let ssid = env::var(ENV_HOTSPOT_SSID)
            .unwrap_or_else(|_| String::from(DEFAULT_HOTSPOT_SSID));
        let password = env::var(ENV_HOTSPOT_PASSWORD)
            .unwrap_or_else(|_| String::from(DEFAULT_HOTSPOT_PASSWORD));

        // Ensure the password is long enough
        if password.len() < 8 {
            error!("The password provided for the hotspot by the DRIVEPI_HOTSPOT_PASSWORD environment");
            error!("variable is too short. Minimum length required is 8 characters.");
            exit(1);
        }

        // Create the hotspot using the nmcli (network-manager cli) tool
        let output = Command::new("nmcli")
            .args([
                "device", "wifi", "hotspot",
                "ifname", interface.as_ref(),
                "con-name", HOTSPOT_CONN_NAME,
                "ssid", ssid.clone().as_ref(),
                "password", password.clone().as_ref()
            ])
            .output()
            .unwrap_or_else(|err| {
                error!("Failed to start hotspot: {}", err);
                exit(2);
            });

        // Parse the stdout as a string
        let output = String::from_utf8(output.stdout)
            .unwrap_or_else(|err| {
                error!("Failed to parse nmcli output: {}", err);
                exit(3);
            });

        // Fail if the message doesn't say success
        if !output.contains("successfully activated") {
            error!("Failed to start hotspot: {}", output);
            exit(4);
        }

        info!("Created hotspot named {} with password {}", ssid, password);
        return Self {};
    }

    fn stop(&self) {
        // Stop the hotspot using the nmcli (network-manager cli) tool
        Command::new("nmcli")
            .args(["con", "down", HOTSPOT_CONN_NAME, ])
            .output()
            .expect("Failed to stop hotspot");
    }
}

impl Drop for Hotspot {
    fn drop(&mut self) {
        self.stop()
    }
}