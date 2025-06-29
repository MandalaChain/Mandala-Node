#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

ROOT=$(get_project_root)
CHAIN_TYPE=$1
ZOMBIENET_PATH=$ROOT/.maintain/zombienet/binaries/zombienet

if [ -z "$1" ]; then
    echo "Usage: ./start-zombienet.sh <chain-type> [zombienet-path]"
    echo "  <chain-type>: local, mainnet, dev, or live"
    echo "  [zombienet-path]: Optional path to zombienet binary (default: $ZOMBIENET_PATH)"
    exit 1
fi

if [ -z "$2" ]; then
    print_status "Using default zombienet path: $ZOMBIENET_PATH"
else
    ZOMBIENET_PATH=$2
fi

# Check if zombienet binary exists
if [ ! -f "$ZOMBIENET_PATH" ]; then
    print_error "Zombienet binary not found at $ZOMBIENET_PATH"
    print_status "Please run: .maintain/scripts/download-zombienet.sh"
    exit 1
fi

# Check if polkadot binary exists
POLKADOT_PATH=$ROOT/.maintain/zombienet/binaries/polkadot
if [ ! -f "$POLKADOT_PATH" ]; then
    print_error "Polkadot binary not found at $POLKADOT_PATH"
    print_status "Please run: .maintain/scripts/download-polkadot-binaries.sh"
    print_status "Note: This will download pre-built binaries (faster than compiling)"
    exit 1
fi

case $CHAIN_TYPE in
    dev|live)
        CHAIN_DIR="niskala"
        CONFIG_FILE="config-niskala.toml"
        ;;
    local|mainnet)
        CHAIN_DIR="mandala"
        CONFIG_FILE="config.toml"
        ;;
    *)
        print_error "Invalid chain type. Please use local, mainnet, dev, or live."
        exit 1
        ;;
esac

CHAIN_PATH=$ROOT/res/$CHAIN_DIR/$CHAIN_TYPE/$CHAIN_TYPE.json

cd $ROOT

print_status "Updating cargo dependencies..."
cargo update

print_status "Building parachain binary for $CHAIN_TYPE..."
case $CHAIN_TYPE in
    dev|live)
        cargo build --release --features niskala-native
        ;;
    local|mainnet)
        cargo build --release --features mandala-native
        ;;
esac

# Make mandala binary executable and copy to binaries directory
chmod +x ./target/release/mandala
mkdir -p $ROOT/.maintain/zombienet/binaries
cp ./target/release/mandala $ROOT/.maintain/zombienet/binaries/

print_status "Generating chain specification..."
# Create the res directory structure if it doesn't exist
mkdir -p $ROOT/res/$CHAIN_DIR/$CHAIN_TYPE

# Generate the appropriate chain spec
if [[ "$CHAIN_TYPE" == "dev" ]] || [[ "$CHAIN_TYPE" == "live" ]]; then
    # For niskala chains, use the dev predefined spec
    print_status "Generating fresh chainspec for $CHAIN_TYPE..."
    ./target/release/mandala build-spec --chain dev > $CHAIN_PATH
elif [[ "$CHAIN_TYPE" == "local" ]] || [[ "$CHAIN_TYPE" == "mainnet" ]]; then
    # For mandala chains, always regenerate from dev template
    print_status "Generating fresh chainspec for $CHAIN_TYPE based on dev template..."
    ./target/release/mandala build-spec --chain dev > $CHAIN_PATH.tmp
    
    # If an existing chainspec exists, we might want to preserve some settings
    if [ -f "$CHAIN_PATH" ]; then
        print_warning "Note: Overwriting existing chainspec at $CHAIN_PATH"
        # For now, just use the fresh one
        mv $CHAIN_PATH.tmp $CHAIN_PATH
    else
        mv $CHAIN_PATH.tmp $CHAIN_PATH
    fi
fi

# Always regenerate raw chain spec to ensure compatibility
RAW_CHAIN_PATH=${CHAIN_PATH%.json}-raw.json
print_status "Generating raw chain specification..."
./target/release/mandala build-spec --chain $CHAIN_PATH --raw > $RAW_CHAIN_PATH

cd $ROOT/.maintain/zombienet
cp $CHAIN_PATH .

# Export genesis state and wasm for manual parachain registration
print_status "Exporting genesis state and wasm..."
$ROOT/.maintain/zombienet/binaries/mandala export-genesis-state --chain ${CHAIN_PATH%.json}-raw.json > $ROOT/.maintain/zombienet/binaries/genesis-state-$CHAIN_TYPE
$ROOT/.maintain/zombienet/binaries/mandala export-genesis-wasm --chain ${CHAIN_PATH%.json}-raw.json > $ROOT/.maintain/zombienet/binaries/genesis-wasm-$CHAIN_TYPE

print_warning "Starting Zombienet with configuration: $CONFIG_FILE"

PATH=$ROOT/.maintain/zombienet/binaries:$PATH CHAIN=$CHAIN_TYPE.json $ZOMBIENET_PATH spawn $ROOT/.maintain/zombienet/$CONFIG_FILE --provider native

rm $CHAIN_TYPE.json
