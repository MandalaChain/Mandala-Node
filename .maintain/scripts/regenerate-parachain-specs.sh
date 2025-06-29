#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

ROOT=$(get_project_root)
cd $ROOT

print_status "Building parachain binaries with latest code..."

# Build Mandala runtime
print_status "Building Mandala runtime..."
cargo build --release --features mandala-native

# Build Niskala runtime  
print_status "Building Niskala runtime..."
cargo build --release --features niskala-native

MANDALA_BINARY=$ROOT/target/release/mandala

# Regenerate Mandala chainspecs
print_status "Regenerating Mandala chainspecs..."

# Local chainspec
if [ -f "$ROOT/res/mandala/local/local.json" ]; then
    print_status "Regenerating raw local chainspec from existing local.json..."
    $MANDALA_BINARY build-spec --chain $ROOT/res/mandala/local/local.json --raw > $ROOT/res/mandala/local/raw-local.json
    
    # Export genesis state and wasm
    print_status "Exporting genesis state and wasm for local..."
    $MANDALA_BINARY export-genesis-state --chain $ROOT/res/mandala/local/raw-local.json > $ROOT/res/mandala/local/state/genesis-state.json
    $MANDALA_BINARY export-genesis-wasm --chain $ROOT/res/mandala/local/raw-local.json > $ROOT/res/mandala/local/state/genesis-wasm.wasm
else
    print_warning "Local chainspec not found, generating from dev template..."
    $MANDALA_BINARY build-spec --chain dev > $ROOT/res/mandala/local/local.json
    $MANDALA_BINARY build-spec --chain $ROOT/res/mandala/local/local.json --raw > $ROOT/res/mandala/local/raw-local.json
fi

# Mainnet chainspec
if [ -f "$ROOT/res/mandala/mainnet/mainnet.json" ]; then
    print_status "Regenerating raw mainnet chainspec from existing mainnet.json..."
    $MANDALA_BINARY build-spec --chain $ROOT/res/mandala/mainnet/mainnet.json --raw > $ROOT/res/mandala/mainnet/raw-mainnet.json
    
    # Export genesis state and wasm
    print_status "Exporting genesis state and wasm for mainnet..."
    mkdir -p $ROOT/res/mandala/mainnet/state
    $MANDALA_BINARY export-genesis-state --chain $ROOT/res/mandala/mainnet/raw-mainnet.json > $ROOT/res/mandala/mainnet/state/genesis-state.json
    $MANDALA_BINARY export-genesis-wasm --chain $ROOT/res/mandala/mainnet/raw-mainnet.json > $ROOT/res/mandala/mainnet/state/genesis-wasm.wasm
else
    print_warning "Mainnet chainspec not found, skipping..."
fi

# Regenerate Niskala chainspecs
print_status "Regenerating Niskala chainspecs..."

# Dev chainspec
print_status "Regenerating dev chainspec..."
$MANDALA_BINARY build-spec --chain dev > $ROOT/res/niskala/dev/dev.json
$MANDALA_BINARY build-spec --chain $ROOT/res/niskala/dev/dev.json --raw > $ROOT/res/niskala/dev/dev-raw.json

# Live chainspec
if [ -f "$ROOT/res/niskala/live/live.json" ]; then
    print_status "Regenerating raw live chainspec from existing live.json..."
    $MANDALA_BINARY build-spec --chain $ROOT/res/niskala/live/live.json --raw > $ROOT/res/niskala/live/live-raw.json
else
    print_warning "Live chainspec not found, skipping..."
fi

print_success "Parachain chainspecs regenerated successfully!"
print_warning "Note: If you modified runtime code, make sure to:"
print_warning "  1. Update spec_version in runtime/lib.rs"
print_warning "  2. Review any changes to genesis config"
print_warning "  3. Test the new chainspecs thoroughly"