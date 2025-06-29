#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Function to print usage
print_usage() {
    echo "Usage: $0 [-w|--which version]"
    echo "  -w, --which version    Specify Rust version (e.g., 1.70.0). If not provided, latest stable version will be installed."
}

# Parse command line options
while getopts ":w:-:" opt; do
    case ${opt} in
        w )
            RUST_VERSION=$OPTARG
            ;;
        - )
            case "${OPTARG}" in
                which=* )
                    RUST_VERSION=${OPTARG#*=}
                    ;;
                which )
                    RUST_VERSION="$2"
                    shift
                    ;;
                * )
                    print_error "Invalid option: --$OPTARG" >&2
                    print_usage
                    exit 1
                    ;;
            esac
            ;;
        \? )
            print_usage
            exit 1
            ;;
        : )
            print_error "Error: -$OPTARG requires an argument."
            print_usage
            exit 1
            ;;
    esac
done

# Detect operating system
OS=$(detect_os)
if [[ "$OS" == "unknown" ]]; then
    print_error "Unsupported operating system: $OSTYPE"
    exit 1
fi

# Call the appropriate OS-specific script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OS_SCRIPT="$SCRIPT_DIR/install-$OS-deps.sh"

if [ -f "$OS_SCRIPT" ]; then
    # Make the OS-specific script executable
    chmod +x "$OS_SCRIPT"
    # Run the OS-specific script
    "$OS_SCRIPT" "$RUST_VERSION"
else
    print_error "OS-specific script not found: $OS_SCRIPT"
    exit 1
fi

# Verify installation
print_status "Verifying Rust installation..."
rustc --version
cargo --version
rustup --version

print_status "Configuring Rust toolchain..."
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

print_status "Adding nightly release and WebAssembly target..."
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

print_status "Verifying configuration..."
rustup show
rustup +nightly show

print_status "Rust toolchain installation and configuration completed successfully!"
