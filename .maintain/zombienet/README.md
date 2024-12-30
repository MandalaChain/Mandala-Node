# Zombienet Testing Infrastructure

## Overview

This directory contains the Zombienet configuration and testing infrastructure for running a local test network with relay chain and parachain nodes.

## Directory Structure

```
zombienet/
├── README.md           # This documentation
├── configs/            # Configuration files
│   ├── json/          # JSON configuration variants
│   │   └── config.json
│   └── toml/          # TOML configuration variants
│       └── config.toml
└── binaries/          # Required binary files
    └── .gitkeep
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

1. Download Zombienet:
```bash
.maintain/scripts/download-zombienet.sh
```

2. Compile Mandala Polkadot:
```bash
.maintain/scripts/compile-mandala-polkadot.sh
```

3. Start the network:
```bash
.maintain/scripts/start-zombienet.sh <chain-type>
```

### Chain Types
- `local`: Local development chain (Mandala)
- `mainnet`: Production chain (Mandala)
- `dev`: Development chain (Niskala)
- `live`: Live network (Niskala)

## Troubleshooting

1. Binary Missing:
   - Verify all required binaries are in `binaries/`
   - Re-run compilation scripts if needed

2. Port Conflicts:
   - Check port availability
   - Modify port numbers in config files if needed

3. Network Connection:
   - Verify relay chain is running
   - Check parachain registration
   - Review logs for connection errors

## Additional Resources

- [Zombienet GitHub](https://github.com/paritytech/zombienet)
- [Polkadot Wiki](https://wiki.polkadot.network)
