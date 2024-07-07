#!/bin/bash

set -euxo pipefail

# Get the latest version
LATEST_VERSION=$(curl -s https://api.github.com/repos/Enter-tainer/hubakc/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "Unable to fetch the latest version. Please check your internet connection."
    exit 1
fi

# Download the latest version of hubakc
wget "https://github.com/Enter-tainer/hubakc/releases/download/${LATEST_VERSION}/hubakc"

if [ $? -ne 0 ]; then
    echo "Download failed. Please check your internet connection."
    exit 1
fi

# Set execute permissions
chmod 755 hubakc

# Change owner to root
sudo chown root:root hubakc

# Move to /usr/local/bin/
sudo mv hubakc /usr/local/bin/

# Create config directory
sudo mkdir -p /etc/hubakc

# Create config file
sudo tee /etc/hubakc/config.toml > /dev/null << EOL
ttl = 3600
timeout = 10
cache_folder = "/tmp/hubakc"
[user_map]
mgt = "Enter-tainer"
EOL

echo "Installation and configuration completed!"
echo "Installed hubakc version: ${LATEST_VERSION}"
echo "WARNING: The default configuration is for the author's personal use."
echo "Please review and modify the configuration file at /etc/hubakc/config.toml to suit your needs."
