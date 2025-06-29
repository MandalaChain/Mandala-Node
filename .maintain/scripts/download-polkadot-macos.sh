#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Function to print usage
print_usage() {
    echo "Usage: $0 [version]"
    echo "Downloads official Polkadot binaries from GitHub releases for macOS."
    echo "Example: $0 v1.11.0"
    echo "If no version is specified, uses the latest stable release."
}

# Check if we're in the project root
if ! check_project_root; then
    print_usage
    exit 1
fi

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    print_error "This script is for macOS only."
    print_status "For Linux, use: .maintain/scripts/download-polkadot-binaries.sh"
    exit 1
fi

# Set version (use provided version or default)
# stable2412 corresponds to v1.16.x releases
VERSION=${1:-"stable2412"}
BINARIES_DIR=".maintain/zombienet/binaries"

print_status "Fetching Polkadot binaries for macOS (${VERSION})..."
print_warning "Note: Using stable2412 compatible version for Mandala/Niskala"

# Create binaries directory if it doesn't exist
mkdir -p "$BINARIES_DIR"

# Function to download binary (no extraction needed for stable releases)
download_binary() {
    local BINARY_NAME=$1
    local URL=$2
    local OUTPUT_FILE="$BINARIES_DIR/${BINARY_NAME}"
    
    print_status "Downloading ${BINARY_NAME} from:"
    print_status "  $URL"
    
    # Download the binary directly (not compressed)
    if command -v curl &> /dev/null; then
        curl -L -f -o "$OUTPUT_FILE" "$URL" || {
            print_error "Failed to download ${BINARY_NAME}"
            rm -f "$OUTPUT_FILE"
            return 1
        }
    elif command -v wget &> /dev/null; then
        wget -O "$OUTPUT_FILE" "$URL" || {
            print_error "Failed to download ${BINARY_NAME}"
            rm -f "$OUTPUT_FILE"
            return 1
        }
    else
        print_error "Neither wget nor curl found. Please install one of them."
        exit 1
    fi
    
    # Check if file was downloaded
    if [ ! -f "$OUTPUT_FILE" ] || [ ! -s "$OUTPUT_FILE" ]; then
        print_error "Download failed or file is empty"
        return 1
    fi
    
    # Make executable
    chmod +x "$OUTPUT_FILE"
    print_status "Successfully installed ${BINARY_NAME}"
}

# Detect architecture
ARCH=$(detect_arch)
if [[ "$ARCH" == "x64" ]]; then
    ARCH_SUFFIX="x86_64"
elif [[ "$ARCH" == "arm64" ]]; then
    ARCH_SUFFIX="aarch64"
else
    print_error "Unsupported architecture: $ARCH"
    exit 1
fi

# URLs for the binaries
# Note: For stable releases, binaries are not compressed as tar.gz
BASE_URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-${VERSION}"
POLKADOT_URL="${BASE_URL}/polkadot-${ARCH_SUFFIX}-apple-darwin"
POLKADOT_PARACHAIN_URL="${BASE_URL}/polkadot-parachain-${ARCH_SUFFIX}-apple-darwin"
POLKADOT_EXECUTE_WORKER_URL="${BASE_URL}/polkadot-execute-worker-${ARCH_SUFFIX}-apple-darwin"
POLKADOT_PREPARE_WORKER_URL="${BASE_URL}/polkadot-prepare-worker-${ARCH_SUFFIX}-apple-darwin"

# Download polkadot binary
download_binary "polkadot" "$POLKADOT_URL" || exit 1

# Download polkadot-parachain binary
download_binary "polkadot-parachain" "$POLKADOT_PARACHAIN_URL" || exit 1

# Download worker binaries (they are separate in stable2412)
print_status "Downloading worker binaries..."
download_binary "polkadot-execute-worker" "$POLKADOT_EXECUTE_WORKER_URL" || print_warning "Failed to download execute worker"
download_binary "polkadot-prepare-worker" "$POLKADOT_PREPARE_WORKER_URL" || print_warning "Failed to download prepare worker"

print_status "Successfully downloaded Polkadot binaries!"
print_status "Binaries installed in: $BINARIES_DIR"
print_status "  - polkadot"
print_status "  - polkadot-parachain"
if [ -f "$BINARIES_DIR/polkadot-prepare-worker" ]; then
    print_status "  - polkadot-prepare-worker"
fi
if [ -f "$BINARIES_DIR/polkadot-execute-worker" ]; then
    print_status "  - polkadot-execute-worker"
fi
print_warning "Note: These are official Polkadot binaries (stable2412)."
print_warning "Compatible with Mandala/Niskala runtime."