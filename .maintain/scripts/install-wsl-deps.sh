#!/bin/bash

set -e

RUST_VERSION=$1

# Install dependencies
sudo apt update
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
source $HOME/.cargo/env

# Set Rust version if specified
if [ -n "$RUST_VERSION" ]; then
    rustup default $RUST_VERSION
fi

echo "Rust installation for Windows (WSL) completed."