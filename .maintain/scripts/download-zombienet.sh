#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Function to print usage
print_usage() {
    echo "Usage: $0"
    echo "Downloads the Zombienet binary for your operating system."
    echo "Run this script from the root of the project."
    echo "Note: On Windows, you need to run this script using Windows Subsystem for Linux (WSL)."
}

# Check if we're in the project root
if ! check_project_root; then
    print_usage
    exit 1
fi

# Detect CPU architecture and OS
ARCH=$(detect_arch)
OS=$(detect_os)

# Validate architecture
case $ARCH in
    x64|arm64)
        # Supported architectures
        ;;
    *)
        print_error "Unsupported CPU architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS for Zombienet
case $OS in
    linux|wsl)
        OS="linux"
        ;;
    macos)
        OS="macos"
        ;;
    windows)
        print_error "This script cannot be run directly on Windows."
        print_info "Please use Windows Subsystem for Linux (WSL) to run this script."
        exit 1
        ;;
    *)
        print_error "Unsupported operating system: $OS"
        print_info "If you're on Windows, please use Windows Subsystem for Linux (WSL) to run this script."
        exit 1
        ;;
esac

# Set Zombienet version
# Note: v1.3.109 is the working version during testing
ZOMBIENET_VERSION="v1.3.109"

# Set download URL based on OS and architecture
case $OS in
    linux)
        DOWNLOAD_URL="https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/zombienet-linux-${ARCH}"
        ;;
    macos)
        DOWNLOAD_URL="https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/zombienet-macos-${ARCH}"
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
if download_file "$DOWNLOAD_URL" "$BINARIES_DIR/zombienet.tmp" "Zombienet ${ZOMBIENET_VERSION}"; then
    # Move the temporary file to the final location
    mv "$BINARIES_DIR/zombienet.tmp" "$BINARIES_DIR/zombienet"
    
    # Make the binary executable
    if make_executable "$BINARIES_DIR/zombienet"; then
        print_status "Zombienet binary downloaded and installed successfully in $BINARIES_DIR"
        print_info "Version: ${ZOMBIENET_VERSION}"
        print_info "Architecture: ${OS}-${ARCH}"
        print_info "You can run it using: $BINARIES_DIR/zombienet"
    else
        exit 1
    fi
else
    print_error "Failed to download Zombienet binary"
    exit 1
fi
