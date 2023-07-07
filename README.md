# do-ddns
A dynamic dns application for DigitalOcean DNS

## Installation
1. Build source code
```
cargo build --release
```
2. Run install.sh with sudo
```
sudo ./install.sh
```
3. Set configuration in `/etc/do-ddns/config.toml`
```toml
# Create the schedule on which to update the DNS record
# sec   min   hour   day of month   month   day of week   year
# *     *     *      *              *       *             *
schedule = "0 1/5 * * * * *"

# The domain managed by DigitalOcean DNS
domain = "example.com"

# The A Record to apply dynamic DNS
# Example: "home" -> home.example.com
record_name = "home" 

# Get auth token from https://cloud.digitalocean.com/account/api/tokens
do_token = "AUTH_TOKEN"
```
4. Restart the ddns service
```
sudo systemctl restart do-ddns
```

## License
This repository uses the Apache License 2.0.
