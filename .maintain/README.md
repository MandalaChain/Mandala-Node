# Maintenance Scripts and Tools

This directory contains various maintenance scripts and tools for setting up and managing the development environment, particularly focused on Zombienet configuration and testing infrastructure.

## Directory Structure

```
.maintain/
├── scripts/           # Maintenance and setup scripts
│   ├── compile-mandala-polkadot.sh
│   ├── download-zombienet.sh
│   ├── install-rust-toolchain.sh
│   ├── install-linux-deps.sh
│   ├── install-macos-deps.sh
│   ├── install-wsl-deps.sh
│   └── start-zombienet.sh
└── zombienet/        # Zombienet testing infrastructure
    ├── README.md     # Zombienet-specific documentation
    ├── configs/      # Configuration files
    │   ├── json/     # JSON configuration variants
    │   └── toml/     # TOML configuration variants
    └── binaries/     # Required binary files
```

## Quick Start

1. Install Dependencies:
```bash
.maintain/scripts/install-rust-toolchain.sh
```

2. Download Zombienet:
```bash
.maintain/scripts/download-zombienet.sh
```

3. Compile Polkadot:
```bash
.maintain/scripts/compile-mandala-polkadot.sh
```

4. Start Testing Environment:
```bash
.maintain/scripts/start-zombienet.sh <chain-type>
```

## Scripts Overview

### Environment Setup

#### `install-rust-toolchain.sh`
Installs and configures the Rust toolchain with necessary targets and components.
```bash
./install-rust-toolchain.sh [-w|--which version]
```

#### `install-{os}-deps.sh`
OS-specific dependency installation scripts:
- Linux: `install-linux-deps.sh`
- macOS: `install-macos-deps.sh`
- WSL: `install-wsl-deps.sh`

### Zombienet Configuration

#### `compile-mandala-polkadot.sh`
Compiles the Mandala Polkadot binary from source.

#### `download-zombienet.sh`
Downloads the Zombienet binary for your operating system.

#### `start-zombienet.sh`
Launches the Zombienet testing environment.
```bash
./start-zombienet.sh <chain-type> [zombienet-path]
```

## Zombienet Testing Infrastructure

The `zombienet/` directory contains all necessary configurations and binaries for running a test network. See [zombienet/README.md](.maintain/zombienet/README.md) for detailed documentation.

### Chain Types
- `local`: Local development chain (Mandala)
- `mainnet`: Production chain (Mandala)
- `dev`: Development chain (Niskala)
- `live`: Live network (Niskala)

## Contributing

When adding new scripts or modifying existing ones:
1. Maintain consistent error handling
2. Add appropriate documentation
3. Test across all supported platforms
4. Update relevant README files

## Additional Resources

- [Zombienet Documentation](https://github.com/paritytech/zombienet)
- [Polkadot Documentation](https://wiki.polkadot.network)
- [Substrate Documentation](https://docs.substrate.io)
