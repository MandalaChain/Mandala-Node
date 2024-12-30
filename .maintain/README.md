# Maintenance Scripts and Tools

This directory contains various maintenance scripts and tools for setting up and managing the development environment, particularly focused on Zombienet configuration and testing infrastructure.

## Directory Structure

```
.maintain/
├── scripts/
│   ├── compile-mandala-polkadot.sh
│   ├── download-zombienet.sh
│   ├── install-rust-toolchain.sh
│   ├── install-linux-deps.sh
│   ├── install-macos-deps.sh
│   ├── install-wsl-deps.sh
│   └── start-zombienet.sh
```

## Scripts Overview

### Environment Setup

#### `install-rust-toolchain.sh`
Installs and configures the Rust toolchain with necessary targets and components.

**Usage:**
```bash
./install-rust-toolchain.sh [-w|--which version]
```
**Features:**
- Installs specified Rust version or latest stable
- Configures WASM target
- Sets up nightly toolchain
- OS-specific dependency installation

#### `install-{os}-deps.sh`
OS-specific dependency installation scripts for Linux, macOS, and WSL.

**Supported Platforms:**
- Linux: `install-linux-deps.sh`
- macOS: `install-macos-deps.sh`
- WSL: `install-wsl-deps.sh`

### Zombienet Configuration

#### `compile-mandala-polkadot.sh`
Compiles the Mandala Polkadot binary from source.

**Features:**
- Clones from MandalaChain/polkadot-sdk repository
- Uses specific branch: mandala-polkadot-v1.11.0
- Builds and installs binaries to zombienet/binaries/
- Handles both fresh installs and updates

#### `download-zombienet.sh`
Downloads the Zombienet binary for your operating system.

**Features:**
- Automatic OS detection
- Downloads version v1.3.109 (tested working version)
- Supports Linux, macOS, and WSL
- Handles binary permissions

#### `start-zombienet.sh`
Launches the Zombienet testing environment.

**Usage:**
```bash
./start-zombienet.sh <chain-type> [zombienet-path]
```

**Chain Types:**
- `local`: Local development chain (Mandala)
- `mainnet`: Production chain (Mandala)
- `dev`: Development chain (Niskala)
- `live`: Live network (Niskala)

**Features:**
- Automatic chain spec generation
- Conditional compilation based on chain type
- Environment setup for testing

## Zombienet Configuration Details

### Network Architecture

#### Relay Chain Configuration
- **Nodes:**
  - Alice (Primary Validator)
    - WebSocket Port: 9944
    - P2P Port: 30444
  - Bob (Secondary Validator)
    - WebSocket Port: 9955
    - P2P Port: 30555

#### Parachain Configuration
- **Chain ID:** 200 (JSON) / 2000 (TOML)
- **Collator Node:**
  - WebSocket Port: 9988
  - P2P Port: 31200
  - Flags: force-authoring, WASM execution

### Configuration Files

#### JSON Configuration (`zombienet/config.json`)
- Detailed relay chain setup
- Parachain node configuration
- Network parameters
- Initial balance settings

#### TOML Configuration (`zombienet/config.toml`)
- Network timeout settings
- Chain specifications
- Node roles and properties
- Execution parameters

## Development Workflow

1. **Initial Setup:**
```bash
# Install Rust and dependencies
.maintain/scripts/install-rust-toolchain.sh

# Download Zombienet
.maintain/scripts/download-zombienet.sh

# Compile Polkadot
.maintain/scripts/compile-mandala-polkadot.sh
```

2. **Start Testing Environment:**
```bash
# For local development
.maintain/scripts/start-zombienet.sh local

# For mainnet testing
.maintain/scripts/start-zombienet.sh mainnet
```

3. **Verify Network:**
- Check WebSocket endpoints
- Verify validator connections
- Monitor parachain registration

## Troubleshooting

### Common Issues

1. **Binary Missing:**
   - Ensure all required binaries are in `zombienet/binaries/`
   - Re-run compilation scripts if needed

2. **Port Conflicts:**
   - Check for port availability
   - Modify port numbers in config files if needed

3. **Network Connection:**
   - Verify relay chain is running
   - Check parachain registration
   - Review logs for connection errors

### Debug Commands

```bash
# Check binary versions
./zombienet/binaries/polkadot --version
./zombienet/binaries/zombienet --version

# Verify chain spec
./target/release/mandala build-spec --chain dev

# Check network status
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944
```

## Additional Resources

- [Zombienet Documentation](https://github.com/paritytech/zombienet)
- [Polkadot Documentation](https://wiki.polkadot.network)
- [Substrate Documentation](https://docs.substrate.io)

## Contributing

When adding new scripts or modifying existing ones:
1. Maintain consistent error handling
2. Add appropriate documentation
3. Test across all supported platforms
4. Update this README as needed
