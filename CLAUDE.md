# Mandala Node Development Guide

## Overview
Mandala Node is a Substrate-based parachain with two runtime configurations:
- **Mandala**: The main chain (local/mainnet environments)
- **Niskala**: The test chain (dev/live environments)

## Quick Start

### Prerequisites
- Rust toolchain
- Git
- macOS or Linux

### Building the Node
```bash
cargo build --release --features mandala-native  # For Mandala runtime
cargo build --release --features niskala-native  # For Niskala runtime
```

### Running with Zombienet (Recommended)
```bash
# Download required binaries
.maintain/scripts/download-zombienet.sh
.maintain/scripts/download-polkadot-binaries.sh

# Start a local testnet
.maintain/scripts/start-zombienet.sh local    # Mandala local
.maintain/scripts/start-zombienet.sh dev      # Niskala dev
.maintain/scripts/start-zombienet.sh live     # Niskala live
.maintain/scripts/start-zombienet.sh mainnet  # Mandala mainnet
```

## Chain Configurations

### Directory Structure
```
res/
├── mandala/
│   ├── local/      # Rococo-local relay, Para ID: 2000, XCM v5
│   └── mainnet/    # Polkadot relay, Para ID: 3366, XCM v4
└── niskala/
    ├── dev/        # Rococo-local relay, Para ID: 2000, XCM v4
    └── live/       # Paseo relay, Para ID: 4022, XCM v4
```

### Relay Chain Compatibility
The project uses `moonbeam-polkadot-stable2412` for all Substrate dependencies.

## Important Scripts

### gen_state.sh
Generates chain specifications and genesis state:
```bash
./gen_state.sh local    # Generate for specific chain
./gen_state.sh all      # Generate for all chains
```

### start-zombienet.sh
Builds the parachain, generates specs, and launches a complete local testnet:
- Automatically compiles the correct runtime
- Generates fresh chain specifications
- Exports genesis state and wasm
- Starts relay chain validators and parachain collators

## Network Endpoints

### Local/Dev Environment
- Relay Chain Alice: `ws://localhost:9944`
- Relay Chain Bob: `ws://localhost:9945`
- Parachain Collator: `ws://localhost:9988` (Mandala)
- Parachain Collator 1: `ws://localhost:9946` (Niskala)
- Parachain Collator 2: `ws://localhost:9947` (Niskala)

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests with Zombienet
1. Start zombienet: `.maintain/scripts/start-zombienet.sh local`
2. Wait for parachain to produce blocks
3. Connect to endpoints using Polkadot.js or similar tools

## Docker Support
Build and run using the Containerfile:
```bash
docker build -t mandala-node .
docker run -p 9944:9944 -p 30333:30333 mandala-node
```

## Key Commands to Remember

### Linting and Type Checking
```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo check --all-features
```

### Clean Build
```bash
cargo clean
rm -rf .maintain/zombienet/binaries/genesis-*
rm -rf .maintain/zombienet/binaries/mandala
```

## Troubleshooting

### Zombienet Issues
- Ensure binaries are downloaded: `.maintain/scripts/download-*.sh`
- Check ports aren't already in use
- Verify correct runtime features are enabled

### Chain Spec Issues
- Always regenerate raw specs after modifying chain configs
- Ensure genesis state matches the raw chain spec
- Check para ID matches between chain spec and zombienet config

## Notes for Future Development
- When updating relay chain version, check all `res/*/` JSON files for compatibility
- Zombienet configs use `rococo-local` for development, `paseo` for testnet, `polkadot` for mainnet
- The project supports both EVM and Substrate features
- Always test with zombienet before deploying changes