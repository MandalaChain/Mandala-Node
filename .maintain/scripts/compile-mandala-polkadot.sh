#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Function to print usage
print_usage() {
    echo "Usage: $0"
    echo "Compiles the Mandala Polkadot binary from the MandalaChain/polkadot-sdk repository."
    echo "Run this script from the root of the project."
}

# Set variables
REPO_URL="https://github.com/MandalaChain/polkadot-sdk"
BRANCH="mandala-polkadot-v1.11.0"
TMP_DIR="./tmp/mandala-polkadot-build"
BINARIES_DIR=".maintain/zombienet/binaries"

# Detect architecture for Apple Silicon
ARCH=$(detect_arch)
OS=$(detect_os)
if [[ "$OS" == "macos" ]] && [[ "$ARCH" == "arm64" ]]; then
    print_info "Detected Apple Silicon (M1/M2/M3) - will compile with optimizations"
fi

# Get the absolute path of the project root before changing directories
PROJECT_ROOT=$(get_project_root)
ABSOLUTE_BINARIES_DIR="$PROJECT_ROOT/$BINARIES_DIR"

# Create and navigate to temporary directory
mkdir -p "$TMP_DIR"
cd "$TMP_DIR"

print_status "Checking if repository already exists..."
if [ "$(ls -A .)" ]; then
    print_status "Directory not empty. Attempting to update existing repository..."
    if [ -d ".git" ]; then
        git fetch
        git reset --hard origin/$BRANCH
    else
        print_error "Directory is not empty and doesn't appear to be a git repository."
        print_info "Please clear the contents of $TMP_DIR manually and run the script again."
        exit 1
    fi
else
    print_status "Cloning MandalaChain/polkadot-sdk repository..."
    git clone "$REPO_URL" .
fi

print_status "Switching to branch $BRANCH..."
git checkout "$BRANCH"

print_status "Updating Cargo dependencies..."
cargo update

print_status "Building Mandala Polkadot..."
cargo build --release

# Check if binaries were compiled successfully
if [ ! -f "target/release/polkadot" ] || [ ! -f "target/release/polkadot-prepare-worker" ] || [ ! -f "target/release/polkadot-execute-worker" ]; then
    print_error "Compilation failed or binaries not found."
    exit 1
fi

# Make binaries executable
chmod +x target/release/polkadot target/release/polkadot-prepare-worker target/release/polkadot-execute-worker

# Copy binaries to the project's binaries directory
print_status "Copying binaries to $ABSOLUTE_BINARIES_DIR..."
mkdir -p "$ABSOLUTE_BINARIES_DIR"
cp target/release/polkadot target/release/polkadot-prepare-worker target/release/polkadot-execute-worker "$ABSOLUTE_BINARIES_DIR/"

# Navigate back to the project root
cd "$PROJECT_ROOT"

# Keep temporary directory for faster rebuilds
print_status "Build completed. Keeping build directory for faster future builds."
print_info "To clean build directory, run: rm -rf $TMP_DIR"

print_status "Mandala Polkadot binaries have been successfully compiled and copied to $BINARIES_DIR"
print_info "Binaries installed:"
print_info "  - polkadot"
print_info "  - polkadot-prepare-worker"
print_info "  - polkadot-execute-worker"
