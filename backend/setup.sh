#!/bin/bash
# Setup Script for Drive-PI
# Sourced: https://strepo.jacobtread.com/drivepi/boot.sh

# Update Repositories and install upgrades
echo "Updating Repositories"
apt-get update

echo "Installing Updates"
apt-get upgrade -y

# systemd-resolve conflicts with dnsmasq so it must be disabled
echo "Disabling systemd-resolved"
systemctl disable systemd-resolved
echo "Stopping systemd-resolved"
systemstl stop systemd-resolved

# Delete dns resolver config
echo "Move old resolver configs"
mv /etc/resolv.conf /etc/resolv.old.conf

# Write new dns resolver config
echo "Writing new resolver config"
echo "nameserver 8.8.8.8" | tee /etc/resolv.conf

# Install NetworkManager, Samba, and dnsmasq
echo "Installing NetworkManager, Samba, and dnsmasq"
apt-get install -y network-manager samba dnsmasq

# Enable and start network manager
echo "Starting and enabling NetworkManager"
systemctl start NetworkManager
systemctl enable NetworkManager

# Stop and disable dnsmasq so it doesn't conflict with network-manager
echo "Stopping and disabling dnsmasq"
systemctl stop dnsmasq
systemctl disable dnsmasq

# Entry to add to hosts file
hosts_entry="

127.0.0.1 drivepi.local"

# Append drivepi.local to /etc/hosts
echo "Writing hosts file entry"
echo "$hosts_entry" | tee -a /etc/hosts

echo "Writing dnsmasq config file"
# Rename old config
mv  /etc/NetworkManager/dnsmasq-shared.d/hosts.conf  /etc/NetworkManager/dnsmasq-shared.d/hosts.old.conf
# Write dnsmasq config
echo "address=/.local/10.42.0.1" | tee /etc/NetworkManager/dnsmasq-shared.d/hosts.conf

echo "Allow samba through firewall"
# Allow samba through firewall
ufw allow samba

# Variables
repo="https://strepo.jacobtread.com/drivepi/"
path=/bin/drivepi

# Create server directory
mkdir "$path"

echo "Downloading server executable"
# Download server executable
curl -o $path/server $repo/server

# Make server file executable
chmod +x $path/server

# Environment variables for env file
env_data="
# Management Credentials
DRIVEPI_USERNAME=admin
DRIVEPI_PASSWORD=admin

# Server port
DRIVEPI_PORT=80

# Hotspot details
DRIVEPI_HOTSPOT_INTERFACE=wlan0
DRIVEPI_HOTSPOT_SSID=Drive-PI
DRIVEPI_HOTSPOT_PASSWORD=Drive-PI

# Logging settings
RUST_LOG=drivepi=info
RUST_LOG_STYLE=always
"

echo "Writing .env file"
# Write environment .env file
echo "$env_data" | tee $path/.env

# Samba configuration
samba_config="
[global]
   workgroup = WORKGROUP
   server string = %h server (Samba, Ubuntu)
   log file = /var/log/samba/log.%m
   max log size = 1000
   logging = file
   panic action = /usr/share/samba/panic-action %d
   server role = standalone server
   obey pam restrictions = yes
   unix password sync = yes
   passwd program = /usr/bin/passwd %u
   passwd chat = *Enter\snew\s*\spassword:* %n\n *Retype\snew\s*\spassword:* %n\n *password\supdated\ssuccessfully* .
   pam password change = yes
   map to guest = bad user


# Shares the mounted paths to \\drivepi.local\storage
[storage]
   comment = DrivePI share
   path = /bin/drivepi/mount
   read only = no
   browsable = yes
"

echo "Writing samba config"
# Rename old samba config
mv /etc/samba/smb.conf /etc/samba/smb.old.conf
# Write samba config
echo "$samba_config" | tee /etc/samba/smb.conf

# Service contents
service_data="
[Unit]
Description=Drive-PI startup service
Requires=network.target
After=NetworkManager.service

[Service]
ExecStart=/bin/drivepi/start.sh
Restart=on-failure
EnvironmentFile=/etc/environment

[Install]
WantedBy=default.target
"

echo "Creating daemon service"
# Write service file
echo "$service_data" | tee /etc/systemd/system/drivepi.service

# Service startup script
start_data="
#!/bin/bash
nmcli radio wifi on
# Move to drivepi directory
cd /bin/drivepi || exit
# Start the server
sudo ./server
"

# Write startup script
echo "$start_data" | tee $path/start.sh

# Wlan Fix script fixes according to: https://gist.github.com/jjsanderson/ab2407ab5fd07feb2bc5e681b14a537a
# Copy old config
cp /etc/dhcpcd.conf /etc/dhcpcd.old.conf
# Tell dhcpcd to ignore wlan0
echo \"denyinterfaces wlan0\" | tee -a /etc/dhcpcd.conf

network_config="
[main]
plugins=ifupdown,keyfile
dhcp=internal

[ifupdown]
managed=true
"

mv /etc/NetworkManager/NetworkManager.conf /etc/NetworkManager/NetworkManager.old.conf
echo "$network_config" | tee /etc/NetworkManager/NetworkManager.conf
nmcli radio wifi on


# Enable and start drivepi service
systemctl enable drivepi
systemctl start drivepi

reboot