![Logo](assets/exported/svg/logo-side.svg)

# Drive-PI

This application is for a school assessment of mine that requires me to create a Samba share on Raspberry PI that shares
any USB sticks that are plugged into it.
Which should then be accessible through the wireless hotspot that the Raspberry PI will start.

In this case rather than having a physical button and display which shows the IP address and controls the unmounting and
mounting of drives.
This uses a web application which I've created and uses [dnsmasq (https://dnsmasq.org/)](https://dnsmasq.org/) to make
the virtual domain drivepi.local available.

This allows the service to be accessed from the hotspot using the url http://drivepi.local for the web app and the smb
share will be located at
\\drivepi.local\drivepi

## Environment Variables

| Variable                  | Default       | Description                                    |
|---------------------------|---------------|------------------------------------------------|
| DRIVEPI_USERNAME          | admin         | The username to access the web panel           |
| DRIVEPI_PASSWORD          | admin         | The password to access the web panel           |
| DRIVEPI_PORT              | 80            | The port to host the HTTP server on            |
| DRIVEPI_HOTSPOT_INTERFACE | wlan0         | The wireless interface to start the hotspot on |
| DRIVEPI_HOTSPOT_SSID      | Drive-PI      | The SSID / Name of the Hotspot                 |
| DRIVEPI_HOTSPOT_PASSWORD  | Drive-PI      | The Password for accessing the hotspot         |
| RUST_LOG                  | drivepi=info  | Logging crate configuration                    |
| RUST_LOG_STYLE            | always        | Whether to enable colored logging output       |
