#!/bin/bash

set -e

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

# Create and navigate to temporary directory
mkdir -p "$TMP_DIR"
cd "$TMP_DIR"

echo "Checking if repository already exists..."
if [ "$(ls -A .)" ]; then
    echo "Directory not empty. Attempting to update existing repository..."
    if [ -d ".git" ]; then
        git fetch
        git reset --hard origin/$BRANCH
    else
        echo "Error: Directory is not empty and doesn't appear to be a git repository."
        echo "Please clear the contents of $TMP_DIR manually and run the script again."
        exit 1
    fi
else
    echo "Cloning MandalaChain/polkadot-sdk repository..."
    git clone "$REPO_URL" .
fi

echo "Switching to branch $BRANCH..."
git checkout "$BRANCH"

echo "Updating Cargo dependencies..."
cargo update

echo "Building Mandala Polkadot..."
cargo build --release

# Check if binaries were compiled successfully
if [ ! -f "target/release/polkadot" ] || [ ! -f "target/release/polkadot-prepare-worker" ] || [ ! -f "target/release/polkadot-execute-worker" ]; then
    echo "Error: Compilation failed or binaries not found."
    exit 1
fi

# Make binaries executable
chmod +x target/release/polkadot target/release/polkadot-prepare-worker target/release/polkadot-execute-worker

# Copy binaries to the project's binaries directory
echo "Copying binaries to $BINARIES_DIR..."
mkdir -p "$BINARIES_DIR"
cp target/release/polkadot target/release/polkadot-prepare-worker target/release/polkadot-execute-worker "$BINARIES_DIR/"

# Navigate back to the project root
cd -

# Remove temporary directory
echo "Cleaning up..."
rm -rf "$TMP_DIR"

echo "Mandala Polkadot binaries have been successfully compiled and copied to $BINARIES_DIR"
