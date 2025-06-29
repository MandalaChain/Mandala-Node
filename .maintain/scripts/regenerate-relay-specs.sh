#!/bin/bash

set -e

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

ROOT=$(get_project_root)
POLKADOT_BINARY=$ROOT/.maintain/zombienet/binaries/polkadot

if [ ! -f "$POLKADOT_BINARY" ]; then
    print_error "Polkadot binary not found at $POLKADOT_BINARY"
    print_status "Please run: .maintain/scripts/download-polkadot-binaries.sh"
    exit 1
fi

cd $ROOT/.maintain/zombienet

print_status "Regenerating rococo-local relay chain spec..."
$POLKADOT_BINARY build-spec --chain rococo-local --disable-default-bootnode > plain.json

print_status "Generating raw rococo-local relay chain spec..."
$POLKADOT_BINARY build-spec --chain plain.json --raw --disable-default-bootnode > plain-raw.json

print_status "Regenerating paseo-local relay chain spec..."
$POLKADOT_BINARY build-spec --chain paseo-local --disable-default-bootnode > paseo-plain.json

print_status "Generating raw paseo-local relay chain spec..."
$POLKADOT_BINARY build-spec --chain paseo-plain.json --raw --disable-default-bootnode > paseo-plain-raw.json

# Update raw relay configs in the res directories
print_status "Copying raw relay chain specs to res directories..."
cp plain-raw.json $ROOT/res/mandala/local/raw-local-relay.json
cp paseo-plain-raw.json $ROOT/res/niskala/live/paseo-raw.json

print_success "Relay chain specs regenerated successfully!"
print_warning "Updated files:"
print_warning "  - .maintain/zombienet/plain.json (rococo-local)"
print_warning "  - .maintain/zombienet/plain-raw.json (rococo-local raw)"
print_warning "  - .maintain/zombienet/paseo-plain.json (paseo-local)"
print_warning "  - .maintain/zombienet/paseo-plain-raw.json (paseo-local raw)"
print_warning "  - res/mandala/local/raw-local-relay.json"
print_warning "  - res/niskala/live/paseo-raw.json"