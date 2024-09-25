#!/bin/bash

set -e

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
                    echo "Invalid option: --$OPTARG" >&2
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
            echo "Error: -$OPTARG requires an argument."
            print_usage
            exit 1
            ;;
    esac
done

# Detect operating system
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if grep -q Microsoft /proc/version; then
        OS="wsl"
    else
        OS="linux"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported operating system: $OSTYPE"
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
    echo "Error: OS-specific script not found: $OS_SCRIPT"
    exit 1
fi

# Verify installation
echo "Verifying Rust installation..."
rustc --version
cargo --version
rustup --version

echo "Configuring Rust toolchain..."
rustup default stable
rustup update

echo "Adding nightly release and WebAssembly target..."
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

echo "Verifying configuration..."
rustup show
rustup +nightly show

echo "Rust toolchain installation and configuration completed successfully!"
