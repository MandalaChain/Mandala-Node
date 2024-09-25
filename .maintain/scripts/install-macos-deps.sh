#!/bin/bash

set -e

RUST_VERSION=$1

# Install Homebrew if not already installed
if ! command -v brew &> /dev/null; then
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# Install dependencies
brew update
brew install openssl cmake

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to PATH
source $HOME/.cargo/env

# Set Rust version if specified
if [ -n "$RUST_VERSION" ]; then
    rustup default $RUST_VERSION
fi

echo "Rust installation for macOS completed."