# Zombienet Testing Infrastructure

## Overview

This directory contains the Zombienet configuration and testing infrastructure for running a local test network with relay chain and parachain nodes.

## Directory Structure

```
zombienet/
├── README.md           # This documentation
├── config.json         # JSON configuration for Mandala
├── config.toml         # TOML configuration for Mandala
├── config-niskala.toml # TOML configuration for Niskala
├── plain.json          # Plain chain spec
├── plain-raw.json      # Raw chain spec
└── binaries/          # Required binary files (downloaded)
    ├── zombienet
    ├── polkadot
    ├── polkadot-parachain
    ├── polkadot-prepare-worker
    └── polkadot-execute-worker
```

## Prerequisites

The following binaries must be present in the `binaries/` directory:
1. Polkadot binary (relay chain)
2. Zombienet executable
3. Parachain binary

## Configuration Files

### JSON Configuration (configs/json/config.json)
Defines:
- Relay chain setup with two validator nodes (Alice and Bob)
- Parachain configuration with ID 200
- Network ports and WebSocket endpoints
- Initial balance settings

### TOML Configuration (configs/toml/config.toml)
Defines:
- Network timeout settings
- Relay chain configuration
- Parachain setup with ID 2000
- Collator configuration

## Network Architecture

### Relay Chain
- Two validator nodes:
  - Alice (Primary)
    - WebSocket: 9944
    - P2P: 30444
  - Bob (Secondary)
    - WebSocket: 9955
    - P2P: 30555

### Parachain
- Single collator node
- WebSocket: 9988
- P2P: 31200
- Chain ID: 200 (JSON) / 2000 (TOML)

## Usage

### Quick Setup (Recommended)

From the project root, run these commands:

1. **Download Zombienet binary:**
```bash
.maintain/scripts/download-zombienet.sh
```

2. **Download Polkadot binaries:**
```bash
# Automatically detects OS and architecture
.maintain/scripts/download-polkadot-binaries.sh
```

3. **Start the network:**
```bash
.maintain/scripts/start-zombienet.sh <chain-type>
```

This will automatically:
- Build the parachain binary with appropriate features
- Generate chain specifications
- Export genesis state and wasm
- Launch the network with appropriate configuration

### Chain Types
- `local`: Local development chain (Mandala)
- `mainnet`: Production chain (Mandala)
- `dev`: Development chain (Niskala)
- `live`: Live network (Niskala)

## Troubleshooting

### Binary Issues

1. **macOS Quarantine** (Apple Silicon/Intel):
   ```bash
   xattr -d com.apple.quarantine .maintain/zombienet/binaries/*
   ```

2. **Missing Binaries**:
   - Run: `.maintain/scripts/download-polkadot-binaries.sh`
   - For macOS, this automatically uses the macOS-specific script
   - For Linux, it uses Zombienet's setup command

3. **Compilation Issues** (if building from source):
   - The MandalaChain fork may have dependency issues
   - Use pre-built binaries instead (recommended)

### Network Issues

1. **Port Conflicts**:
   - Default ports: 9944, 9945, 9946, 9947, 9988
   - Check if ports are in use: `lsof -i :9944`
   - Modify port numbers in config files if needed

2. **Parachain Not Producing Blocks**:
   - Ensure manual registration was completed
   - Check relay chain is running properly
   - Review parachain collator logs

### Platform-Specific Notes

- **Apple Silicon (M1/M2/M3)**: Scripts automatically detect ARM64 architecture
- **Docker on macOS**: May require additional resources allocation
- **WSL**: Use Linux binary downloads, not Windows native

## Additional Resources

- [Zombienet GitHub](https://github.com/paritytech/zombienet)
- [Polkadot Wiki](https://wiki.polkadot.network)
