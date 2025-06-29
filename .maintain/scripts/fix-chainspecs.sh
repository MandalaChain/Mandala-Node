#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

ROOT=$(get_project_root)
cd $ROOT

print_warning "This script will regenerate all chainspecs to fix compatibility issues"
print_warning "Existing chainspecs will be backed up with .backup extension"

# First, ensure we have the latest binary
print_status "Building parachain binaries with latest code..."
cargo build --release --features mandala-native
cargo build --release --features niskala-native

MANDALA_BINARY=$ROOT/target/release/mandala

# Backup existing chainspecs
print_status "Backing up existing chainspecs..."
for chainspec in $(find $ROOT/res -name "*.json" -type f | grep -E "(local|mainnet|dev|live)\.json$"); do
    if [ -f "$chainspec" ]; then
        cp "$chainspec" "${chainspec}.backup"
        print_status "Backed up: $chainspec"
    fi
done

# Generate fresh Mandala chainspecs
print_status "Generating fresh Mandala chainspecs..."

# For local - use dev as base
mkdir -p $ROOT/res/mandala/local/state
print_status "Generating local chainspec from dev template..."
$MANDALA_BINARY build-spec --chain dev > $ROOT/res/mandala/local/local.json

# Customize the local chainspec (update chain name and id)
sed -i.tmp 's/"id": "dev"/"id": "mandala_local"/' $ROOT/res/mandala/local/local.json
sed -i.tmp 's/"name": "Development"/"name": "Mandala Local Testnet"/' $ROOT/res/mandala/local/local.json
rm -f $ROOT/res/mandala/local/local.json.tmp

# Generate raw
$MANDALA_BINARY build-spec --chain $ROOT/res/mandala/local/local.json --raw > $ROOT/res/mandala/local/raw-local.json

# Export genesis state and wasm
$MANDALA_BINARY export-genesis-state --chain $ROOT/res/mandala/local/raw-local.json > $ROOT/res/mandala/local/state/genesis-state.json
$MANDALA_BINARY export-genesis-wasm --chain $ROOT/res/mandala/local/raw-local.json > $ROOT/res/mandala/local/state/genesis-wasm.wasm

# For mainnet - use dev as base and customize
mkdir -p $ROOT/res/mandala/mainnet/state
print_status "Generating mainnet chainspec from dev template..."
$MANDALA_BINARY build-spec --chain dev > $ROOT/res/mandala/mainnet/mainnet.json

# Customize the mainnet chainspec
sed -i.tmp 's/"id": "dev"/"id": "mandala"/' $ROOT/res/mandala/mainnet/mainnet.json
sed -i.tmp 's/"name": "Development"/"name": "Mandala"/' $ROOT/res/mandala/mainnet/mainnet.json
sed -i.tmp 's/"tokenSymbol": "KPGD"/"tokenSymbol": "KPG"/' $ROOT/res/mandala/mainnet/mainnet.json
sed -i.tmp 's/"tokenDecimals": 12/"tokenDecimals": 18/' $ROOT/res/mandala/mainnet/mainnet.json
rm -f $ROOT/res/mandala/mainnet/mainnet.json.tmp

# Generate raw
$MANDALA_BINARY build-spec --chain $ROOT/res/mandala/mainnet/mainnet.json --raw > $ROOT/res/mandala/mainnet/raw-mainnet.json

# Export genesis state and wasm
$MANDALA_BINARY export-genesis-state --chain $ROOT/res/mandala/mainnet/raw-mainnet.json > $ROOT/res/mandala/mainnet/state/genesis-state.json
$MANDALA_BINARY export-genesis-wasm --chain $ROOT/res/mandala/mainnet/raw-mainnet.json > $ROOT/res/mandala/mainnet/state/genesis-wasm.wasm

# Generate fresh Niskala chainspecs
print_status "Generating fresh Niskala chainspecs..."

# Dev is predefined, just generate it
mkdir -p $ROOT/res/niskala/dev
$MANDALA_BINARY build-spec --chain dev > $ROOT/res/niskala/dev/dev.json
$MANDALA_BINARY build-spec --chain $ROOT/res/niskala/dev/dev.json --raw > $ROOT/res/niskala/dev/dev-raw.json

# For live - use dev as base and customize
mkdir -p $ROOT/res/niskala/live
print_status "Generating live chainspec from dev template..."
$MANDALA_BINARY build-spec --chain dev > $ROOT/res/niskala/live/live.json

# Customize the live chainspec
sed -i.tmp 's/"id": "dev"/"id": "niskala"/' $ROOT/res/niskala/live/live.json
sed -i.tmp 's/"name": "Development"/"name": "Niskala"/' $ROOT/res/niskala/live/live.json
sed -i.tmp 's/"tokenSymbol": "KPGD"/"tokenSymbol": "KPGT"/' $ROOT/res/niskala/live/live.json
rm -f $ROOT/res/niskala/live/live.json.tmp

# Generate raw
$MANDALA_BINARY build-spec --chain $ROOT/res/niskala/live/live.json --raw > $ROOT/res/niskala/live/live-raw.json

# Now regenerate relay chain specs
print_status "Regenerating relay chain specs..."
if [ -f "$ROOT/.maintain/zombienet/binaries/polkadot" ]; then
    POLKADOT_BINARY=$ROOT/.maintain/zombienet/binaries/polkadot
    
    cd $ROOT/.maintain/zombienet
    
    # Generate rococo-local
    $POLKADOT_BINARY build-spec --chain rococo-local --disable-default-bootnode > plain.json
    $POLKADOT_BINARY build-spec --chain plain.json --raw --disable-default-bootnode > plain-raw.json
    
    # Copy to res directories
    cp plain-raw.json $ROOT/res/mandala/local/raw-local-relay.json
    
    print_success "All chainspecs regenerated successfully!"
else
    print_warning "Polkadot binary not found, skipping relay chain spec generation"
    print_warning "Run .maintain/scripts/download-polkadot-binaries.sh to download it"
fi

print_success "Chainspec regeneration complete!"
print_warning "The old chainspecs have been backed up with .backup extension"
print_warning "You can now run: .maintain/scripts/start-zombienet.sh local"