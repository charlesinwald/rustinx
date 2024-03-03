#!/bin/bash

# Replace this URL with the actual URL of your rustinx binary release
# REMEMBER TO UPDATE THE VERSION NUMBER IN THE URL
DOWNLOAD_URL="https://github.com/charlesinwald/rustinx/releases/download/v1.0.0/rustinx"

# Use a system-wide location for the binary
INSTALL_DIR="/usr/local/bin"
BIN_NAME="rustinx"
INSTALL_PATH="$INSTALL_DIR/$BIN_NAME"

echo "Installing rustinx..."

# Create install directory if it doesn't exist
# This operation requires root privileges
sudo mkdir -p "$INSTALL_DIR"

# Download rustinx binary
echo "Downloading rustinx..."
sudo curl -L "$DOWNLOAD_URL" -o "$INSTALL_PATH"

# Make the rustinx binary executable
sudo chmod +x "$INSTALL_PATH"

echo "Installation completed. You can now run rustinx with sudo."
