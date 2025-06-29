#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Script to clean up zombienet data directories
# This helps resolve sync issues with multiple collators

ROOT=$(get_project_root)

print_warning "This will remove all zombienet data directories!"
print_status "This includes relay chain and parachain databases."
read -p "Are you sure you want to continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_error "Aborted."
    exit 1
fi

# Find and remove all zombienet data directories
print_status "Cleaning up zombienet data directories..."

# Remove tmp directories created by zombienet
if [ -d "/tmp/zombie-*" ]; then
    print_status "Removing /tmp/zombie-* directories..."
    rm -rf /tmp/zombie-*
fi

# Remove any local data directories
if [ -d "$ROOT/.maintain/zombienet/data" ]; then
    print_status "Removing local data directory..."
    rm -rf "$ROOT/.maintain/zombienet/data"
fi

# Remove genesis state and wasm files to force regeneration
print_status "Removing cached genesis files..."
rm -f "$ROOT/.maintain/zombienet/binaries/genesis-state-*"
rm -f "$ROOT/.maintain/zombienet/binaries/genesis-wasm-*"

print_status "Cleanup complete!"
print_status "You can now run start-zombienet.sh with a clean state."