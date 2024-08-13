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

### Development Chain
To run a development chain, we first need a relay chain to connect to. In this case, we spin up a 2-node rococo local testnet instance using zombienet. Since zombienet doesn't yet support running ethereun parachain out of the box ([issue](https://github.com/paritytech/zombienet/issues/1826)). So we must manually register our parachain into our Relay chain. To do this, first generate the genesis head and runtime of our node.

```bash
./target/release/mandala export-genesis-state  --dev > <path>
```
```bash
./target/release/mandala export-genesis-wasm --dev > <path>
```
> Replace the path with folder you wish to store the state and runtime

Go to the zombienet folder and run the script
```bash
cd zombienet
./run.sh <zombienet-path>
```
> This will spin up 2 relay node with bob and alice as the validator

Then go the root of the project and run the collator :
```bash 
./target/release/mandala --dev --charlie --collator --rpc-port 9944 --port 30333 -- --chain ./zombienet/plain.json  --discover-local --port 30334 
```

After you run the zombienet script, you should see something like this on your terminal :
![Zombienet Terminal](zombienet_terminal.png)
Click one of the direct link, and it will take you to `polkadotJS` and automatically connect to the node. On the developer tab, go to sudo and select `parasSudoWrapper`. You should see something like below : 
![Alt text](sudoWrapper.png)
Select `sudoScheduleParaInitialize(id, genesis)`. fill the `id` parameter with `2000`. on the `genesisHead` parameter, tick the file upload field and drag your genesis state that you've previously exported. Finally, Set the `paraKind` to `true` and submit the transaction. Wait until the next epoch start and the parachain should produce blocks.

### Connect with Polkadot-JS Apps Front-End

After you start this node locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.
