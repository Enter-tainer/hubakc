#!/bin/bash

set -euxo pipefail

# Get the latest version
LATEST_VERSION=$(curl -s https://api.github.com/repos/Enter-tainer/hubakc/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "Unable to fetch the latest version. Please check your internet connection."
    exit 1
fi

# Determine system architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        BINARY_NAME="hubakc-x86_64-unknown-linux-gnu"
        ;;
    aarch64)
        BINARY_NAME="hubakc-aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Download the latest version of hubakc
wget "https://github.com/Enter-tainer/hubakc/releases/download/${LATEST_VERSION}/${BINARY_NAME}"

if [ $? -ne 0 ]; then
    echo "Download failed. Please check your internet connection."
    exit 1
fi

# Set execute permissions
chmod 755 $BINARY_NAME

# Change owner to root
sudo chown root:root $BINARY_NAME

# Move to /usr/local/bin/
sudo mv $BINARY_NAME /usr/local/bin/hubakc

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
