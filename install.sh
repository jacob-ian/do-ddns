#!/bin/bash
cp ./target/release/do-ddns /usr/local/bin/do-ddns
if [ ! -d "/etc/do-ddns" ]; then
  mkdir /etc/do-ddns
  echo "# Create the schedule on which to update the DNS record
# sec   min   hour   day of month   month   day of week   year
# *     *     *      *              *       *             *
schedule = \"0 1/5 * * * * *\"

# The domain managed by DigitalOcean DNS
domain = \"example.com\"

# The A Record to apply dynamic DNS
# Example: \"home\" -> home.example.com
record_name = \"home\" 

# Get auth token from https://cloud.digitalocean.com/account/api/tokens
do_token = \"AUTH_TOKEN\"" >> /etc/do-ddns/config.toml
  chmod 700 /etc/do-ddns/config.toml
fi
if systemctl --all --type service | grep -q "do-ddns.service"; then    
  systemctl stop do-ddns
fi
echo "[Unit]
Description=A dynamic DNS service for DigitalOcean domains
After=network.target
StartLimitIntervalSec=0
[Service]
Type=simple
Restart=always
RestartSec=1
User=root
Group=root
ExecStart=/usr/local/bin/do-ddns
ExecReload=/bin/kill -HUP \$MAINPID

[Install]
WantedBy=multi-user.target" > /etc/systemd/system/do-ddns.service
systemctl enable do-ddns
systemctl start do-ddns
