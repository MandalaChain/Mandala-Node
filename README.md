# Mandala Node

Mandala chain implementation, build with [Substrate](https://substrate.io/). This repository contains both Mandala and Niskala runtimes.

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this repository.
Check the [Install](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.
Alternatively, you can use one of the [alternative installation](#alternatives-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release --features mandala-native
```
> This will build the node with mandala runtime in it, if you want to build it with niskala runtime instead, compile it with `niskala-native` feature.

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/mandala -h
```

You can generate and view the [Rust Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this repository with this command:

```sh
cargo +nightly doc --open
```

### Development Chain with Zombienet

To run a local development chain with relay chain support, we use Zombienet. Since Zombienet doesn't yet support Ethereum parachains out-of-the-box ([issue](https://github.com/paritytech/zombienet/issues/1826)), manual parachain registration is required.

#### Quick Setup

**Complete setup in 3 commands:**

```bash
# 1. Download Zombienet
.maintain/scripts/download-zombienet.sh

# 2. Download Polkadot binaries (auto-detects OS/architecture)
.maintain/scripts/download-polkadot-binaries.sh

# 3. Start Zombienet (builds parachain and launches network)
.maintain/scripts/start-zombienet.sh local  # For Mandala
# OR
.maintain/scripts/start-zombienet.sh dev    # For Niskala
```

The `start-zombienet.sh` script automatically:
- Detects your OS and architecture (Apple Silicon supported)
- Builds the parachain binary with appropriate features
- Generates chain specifications
- Exports genesis state and wasm files
- Launches a 2-node relay chain network
- Displays connection endpoints

**Prerequisites** (if not already installed):
```bash
# Install Rust
.maintain/scripts/install-rust-toolchain.sh

# macOS users:
.maintain/scripts/install-macos-deps.sh
```

3. **Network endpoints after launch:**
- Relay Chain Alice: `ws://localhost:9944`
- Relay Chain Bob: `ws://localhost:9945`
- Parachain Collator(s): See script output for ports

#### Manual Parachain Registration

After Zombienet starts:

1. Open PolkadotJS Apps: https://polkadot.js.org/apps/?rpc=ws://localhost:9944
2. Navigate to: Developer → Sudo → parasSudoWrapper
3. Select `sudoScheduleParaInitialize(id, genesis)`
4. Fill in the parameters:
   - `id`: `2000`
   - `genesisHead`: Upload file from `.maintain/zombienet/binaries/genesis-state-<chain-type>`
   - `validationCode`: Upload file from `.maintain/zombienet/binaries/genesis-wasm-<chain-type>`
   - `paraKind`: `Yes`
5. Submit the transaction and wait for the next epoch

The parachain should start producing blocks after successful registration.

#### Troubleshooting

- **Apple Silicon users**: Scripts automatically detect and handle ARM64 architecture
- **Binary downloads**: Pre-built binaries are downloaded for faster setup (no compilation needed)
- **Port conflicts**: Check if ports 9944, 9945, etc. are already in use
- **Build from source**: If needed, use `compile-mandala-polkadot.sh` (builds are cached in `tmp/`)
- **Detailed documentation**: See [.maintain/README.md](.maintain/README.md) for comprehensive setup instructions

### Connect with Polkadot-JS Apps Front-End

After you start this node locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.
