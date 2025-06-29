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

## Quick Start - Complete Zombienet Setup

### Prerequisites

1. **Install Rust:**
```bash
.maintain/scripts/install-rust-toolchain.sh
```

2. **Install OS-specific dependencies:**
```bash
# macOS (including Apple Silicon)
.maintain/scripts/install-macos-deps.sh

# Linux
.maintain/scripts/install-linux-deps.sh

# Windows (WSL)
.maintain/scripts/install-wsl-deps.sh
```

### Complete Setup & Execution

#### One-Command Setup (Recommended)

For a complete setup from scratch to running Zombienet:

```bash
# 1. Download Zombienet
.maintain/scripts/download-zombienet.sh

# 2. Download Polkadot binaries (automatically detects OS)
.maintain/scripts/download-polkadot-binaries.sh

# 3. Start Zombienet (builds parachain and launches network)
.maintain/scripts/start-zombienet.sh local  # or: dev, live, mainnet
```

The `start-zombienet.sh` script will:
- Build the parachain binary with appropriate features
- Generate chain specifications
- Export genesis state and wasm
- Launch relay chain validators
- Display connection information

#### Manual Step-by-Step

If you prefer manual control:

1. **Download Zombienet:**
```bash
.maintain/scripts/download-zombienet.sh
```

2. **Download Polkadot binaries:**
```bash
# Automatically detects macOS/Linux and architecture
.maintain/scripts/download-polkadot-binaries.sh
```

3. **Build parachain binary:**
```bash
# For Mandala runtime
cargo build --release --features mandala-native

# For Niskala runtime
cargo build --release --features niskala-native
```

4. **Start Zombienet:**
```bash
.maintain/scripts/start-zombienet.sh <chain-type>
```

### Platform-Specific Notes

#### Apple Silicon (M1/M2/M3)
- Scripts automatically detect ARM64 architecture
- Downloads appropriate arm64 binaries from official releases
- Uses stable2412-compatible binaries (Polkadot v1.17.x)

#### Intel Mac / Linux
- Automatically uses appropriate x86_64 binaries
- Linux can use Zombienet's built-in setup command

#### Architecture Detection
All scripts automatically detect:
- Operating system (macOS/Linux/WSL)
- CPU architecture (ARM64/x86_64)
- And download appropriate binaries

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

The `zombienet/` directory contains all necessary configurations and binaries for running a test network. See [zombienet/README.md](zombienet/README.md) for detailed documentation.

### Chain Types
- `local`: Local development chain (Mandala)
- `mainnet`: Production chain (Mandala)
- `dev`: Development chain (Niskala)
- `live`: Live network (Niskala)

### Running Zombienet

After setup, start Zombienet with:
```bash
.maintain/scripts/start-zombienet.sh <chain-type>
```

This will:
1. Build the parachain binary with appropriate features
2. Generate chain specifications
3. Export genesis state and wasm for parachain registration
4. Launch a 2-node relay chain (Alice & Bob validators)
5. Display connection endpoints

### Network Endpoints

Once running, you can connect to:
- **Relay Chain Alice**: ws://localhost:9944
- **Relay Chain Bob**: ws://localhost:9945
- **Parachain Collator** (Mandala): ws://localhost:9988
- **Parachain Collators** (Niskala): 
  - Collator 1: ws://localhost:9946
  - Collator 2: ws://localhost:9947

### Manual Parachain Registration

Since Zombienet doesn't support Ethereum parachains out-of-the-box, you need to manually register the parachain:

1. Connect to relay chain: https://polkadot.js.org/apps/?rpc=ws://localhost:9944
2. Go to Developer → Sudo → parasSudoWrapper
3. Select `sudoScheduleParaInitialize(id, genesis)`
4. Fill parameters:
   - `id`: 2000
   - `genesisHead`: Upload the genesis state file from `.maintain/zombienet/binaries/genesis-state-<chain-type>`
   - `validationCode`: Upload the wasm file from `.maintain/zombienet/binaries/genesis-wasm-<chain-type>`
   - `paraKind`: true
5. Submit transaction and wait for next epoch

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
