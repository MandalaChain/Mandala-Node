#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Function to print usage
print_usage() {
    echo "Usage: $0"
    echo "Downloads pre-built Polkadot binaries using Zombienet setup command."
    echo "This is faster and more reliable than building from source."
}

# Check if we're in the project root
if ! check_project_root; then
    print_usage
    exit 1
fi

# Set variables
PROJECT_ROOT=$(pwd)
ZOMBIENET_PATH=".maintain/zombienet/binaries/zombienet"
BINARIES_DIR=".maintain/zombienet/binaries"

# Check if zombienet binary exists
if [ ! -f "$ZOMBIENET_PATH" ]; then
    print_error "Zombienet binary not found at $ZOMBIENET_PATH"
    print_status "Please run: .maintain/scripts/download-zombienet.sh"
    exit 1
fi

# Make zombienet executable
chmod +x "$ZOMBIENET_PATH"

# Detect operating system
if [[ "$OSTYPE" == "darwin"* ]]; then
    print_warning "Redirecting to macOS-specific download script..."
    exec $PROJECT_ROOT/.maintain/scripts/download-polkadot-macos.sh "$@"
fi

print_status "Downloading Polkadot binaries using Zombienet setup..."
print_warning "This will download pre-built binaries from the official releases."

# Change to binaries directory
cd "$BINARIES_DIR"

# Run zombienet setup to download polkadot and polkadot-parachain binaries
print_status "Running: zombienet setup polkadot polkadot-parachain"
./zombienet setup polkadot polkadot-parachain

# Check if binaries were downloaded successfully
if [ -f "polkadot" ] && [ -f "polkadot-parachain" ]; then
    print_status "Polkadot binaries downloaded successfully!"
    print_status "Downloaded binaries:"
    print_status "  - polkadot"
    print_status "  - polkadot-parachain"
    
    # The Polkadot workers are typically included with the polkadot binary
    # Let's check if they exist and create symlinks if needed
    if [ -f "polkadot" ]; then
        # Create symlinks for the worker binaries if they don't exist
        if [ ! -f "polkadot-prepare-worker" ]; then
            ln -sf polkadot polkadot-prepare-worker
            print_status "Created symlink for polkadot-prepare-worker"
        fi
        
        if [ ! -f "polkadot-execute-worker" ]; then
            ln -sf polkadot polkadot-execute-worker
            print_status "Created symlink for polkadot-execute-worker"
        fi
    fi
else
    print_error "Failed to download Polkadot binaries"
    exit 1
fi

print_status "All binaries are ready in $BINARIES_DIR"