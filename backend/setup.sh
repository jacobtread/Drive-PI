# SETUP SCRIPT FOR DRIVE-PI
# SOURCED: https://strepo.jacobtread.com/drivepi/setup.sh

# Update the system repositories and run upgrades
apt-get update && apt-get upgrade -y

# systemd-resolve conflicts with dnsmasq so it must be disabled
systemctl disable systemd-resolved
systemctl stop systemd-resolved
unlink /etc/resolv.conf
echo nameserver 8.8.8.8 | tee /etc/resolv.conf

# Install network manager, samba and dnsmasq
apt-get install -y network-manager samba dnsmasq

systemctl start NetworkManager
systemctl enable NetworkManager

# Stop dnsmasq network manager starts it for us
systemctl stop dnsmasq

# Allow samba through the firewall
sudo ufw allow samba

# Move to bin directory
cd /bin || exit
mkdir drivepi
cd drivepi || exit

# Download drivepi server
curl -o server https://strepo.jacobtread.com/drivepi/server
# Replace the local samba config with the drivepi config
curl -o /etc/samba/smb.conf https://strepo.jacobtread.com/drivepi/smb.conf

# Restart samba
sudo service smbd restart

# Execute permission on server
chmod +x server
