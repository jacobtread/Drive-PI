# Update the system repositories and run upgrades
apt-get update && apt-get upgrade

# systemd-resolve conflicts with dnsmasq so it must be disabled
systemctl disable systemd-resolved
systemctl stop systemd-resolved
unlink /etc/resolv.conf
echo nameserver 8.8.8.8 | tee /etc/resolv.conf

# Install network manager, samba and dnsmasq
apt-get install network-manager samba dnsmasq

# Restart dnsmasq
systemctl restart dnsmasq

# Move to bin directory
cd /bin || exit
mkdir drivepi
cd drivepi || exit

# Download drivepi server
curl -o server https://strepo.jacobtread.com/data/drivepi

# Execute permission on server
chmod +x server
