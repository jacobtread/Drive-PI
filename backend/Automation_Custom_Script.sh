
echo "Updating..."
# Update repositories
apt-get update
# Install upgrades without prompting for user input
apt-get upgrade -y


# Disable systemd-resolve as it tries to bind the same
# port as dnsmasq which causes issues
echo "Disabling systemd-resolved..."
systemctl disable systemd-resolved
systemctl stop systemd-resolved

# Install samba and dnsmasq
apt-get install -y samba dnsmasq

# dnsmasq configuration this tells clients that queries for
# the domain drivepi.local should resolve to 192.168.42.1
# which is the router address we will set for our hotspot
dnsmasq_config_lines="

address=/drivepi.local/192.168.42.1
"

# Append the extra lines to the dnsmasq config file
echo "$dnsmasq_config_lines" | tee -a /etc/dnsmasq.conf

# Configuration for the internal DNS server that creates a subnet
# that will use the DNS server created by dnsmasq
# (located at /etc/dhcp/dhcpd.conf)
dhcpd_conf="ddns-update-style none;
default-lease-time 600;
max-lease-time 7200;
authoritative;
log-facility local7;

subnet 192.168.42.0 netmask 255.255.255.0 {
    range 192.168.42.10 192.168.42.50;
    option broadcast-address 192.168.42.255;
    option routers 192.168.42.1;
    option domain-name \"local\";
    option domain-name-servers 192.168.42.1, 8.8.8.8;
}
"

echo "$dhcpd_conf" | tee /etc/dhcp/dhcpd.conf

# Path to Drive-PI install
path=/bin/drivepi

# Create Drive-PI directory to store files
mkdir "$path"

# Download Drive-PI executable from latest GitHub release
echo "Downloading server executable"
curl -L -o $path/server http://github.com/Jacobtread/Drive-PI/releases/latest/download/drivepi
# Change server file permissions to make it executable
chmod +x $path/server

env_config="# Management Credentials
DRIVEPI_USERNAME=admin
DRIVEPI_PASSWORD=admin

# Server port
DRIVEPI_PORT=80

# Logging settings
RUST_LOG=drivepi=info
RUST_LOG_STYLE=always
"

echo "Writing .env file"
# Write environment .env file
echo "$env_config" | tee $path/.env

# Source code for the start.sh script which is started
# by the daemon service created later in this script
start_script="#!/bin/bash
# Move to drivepi directory
cd /bin/drivepi || exit
# Start the server
sudo ./server
"

# Write startup script and make it executable
echo "$start_script" | tee $path/start.sh
chmod +x $path/start.sh

# Samba configuration which is just a simple configuration
# that adds a samba stored named [storage] which shares the
# /bin/drivepi/mount folder which is where Drive-PI mounts
# all its drives to (stored at /etc/samba/smb.conf)
samba_config="[global]
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
   path = $path/mount
   read only = no
   browsable = yes

[guest]
   comment = DrivePI guest share
   path = $path/mount
   read only = yes
   guest ok = yes
   browsable = yes
"

echo "Writing samba config"
# Move the old samba config
mv /etc/samba/smb.conf /etc/samba/smb.old.conf
# Write the new samba config
echo "$samba_config" | tee /etc/samba/smb.conf

# Daemon service for automatically starting the Drive-PI server
# automatically. This is a system service and must be a system
# service because root access is required in order to mount
# and unmount the drives (stored at /etc/systemd/system/drivepi.service)
service_data="[Unit]
Description=Drive-PI application
Requires=network.target
After=NetworkManager.service

[Service]
ExecStart=$path/start.sh
Restart=on-failure
EnvironmentFile=/etc/environment

[Install]
WantedBy=default.target
"

echo "Creating daemon service"
# Write service file
echo "$service_data" | tee /etc/systemd/system/drivepi.service

# Enable drivepi service so that it will automatically start on startup
systemctl enable drivepi

# Force the script to sleep until the install is complete then reboot
(while [ "$(</boot/dietpi/.install_stage)" != 2 ]; do sleep 1; done; /usr/sbin/reboot) > /dev/null 2>&1 &

