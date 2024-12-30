#!/bin/bash

set -e

# Function to print usage
print_usage() {
    echo "Usage: $0"
    echo "Downloads the Zombienet binary for your operating system."
    echo "Run this script from the root of the project."
    echo "Note: On Windows, you need to run this script using Windows Subsystem for Linux (WSL)."
}

# Ensure script is run from project root
if [ ! -d ".maintain" ] || [ ! -d ".maintain/zombienet" ]; then
    echo "Error: This script must be run from the root of the project."
    print_usage
    exit 1
fi

# Detect operating system
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if grep -q Microsoft /proc/version; then
        OS="linux" # Using Linux binary for WSL
    else
        OS="linux"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "Error: This script cannot be run directly on Windows."
    echo "Please use Windows Subsystem for Linux (WSL) to run this script."
    exit 1
else
    echo "Unsupported operating system: $OSTYPE"
    echo "If you're on Windows, please use Windows Subsystem for Linux (WSL) to run this script."
    exit 1
fi

# Set Zombienet version
# Note: v1.3.109 is the working version during testing
ZOMBIENET_VERSION="v1.3.109"

# Set download URL based on OS
case $OS in
    linux)
        DOWNLOAD_URL="https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/zombienet-linux-x64"
        ;;
    macos)
        DOWNLOAD_URL="https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/zombienet-macos-x64"
        ;;
    *)
        echo "Unsupported operating system for Zombienet download"
        exit 1
        ;;
esac

# Set binaries directory relative to project root
BINARIES_DIR=".maintain/zombienet/binaries"

# Create binaries directory if it doesn't exist
mkdir -p "$BINARIES_DIR"

# Remove existing zombienet file or symlink if it exists
if [ -e "$BINARIES_DIR/zombienet" ] || [ -L "$BINARIES_DIR/zombienet" ]; then
    rm -f "$BINARIES_DIR/zombienet"
fi

# Download Zombienet binary
echo "Downloading Zombienet ${ZOMBIENET_VERSION} for ${OS}..."
wget -O "$BINARIES_DIR/zombienet.tmp" "$DOWNLOAD_URL"

# Move the temporary file to the final location
mv "$BINARIES_DIR/zombienet.tmp" "$BINARIES_DIR/zombienet"

# Make the binary executable
chmod +x "$BINARIES_DIR/zombienet"

echo "Zombienet binary downloaded and installed successfully in $BINARIES_DIR"
echo "You can run it using: $BINARIES_DIR/zombienet"
