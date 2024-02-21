# Mandala Node
For now, this is a basic parachain node implementation taken directly from substrate parachain template repository.

## Dev environment (for auction)

```
> rustc --version && rustup show && rustup +nightly show

rustc 1.76.0 (07dca489a 2024-02-04)
Default host: aarch64-apple-darwin
rustup home:  /Users/zian/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin (default)
stable-x86_64-unknown-linux-gnu
nightly-2023-05-31-aarch64-apple-darwin
nightly-aarch64-apple-darwin

installed targets for active toolchain
--------------------------------------

aarch64-apple-darwin
wasm32-unknown-unknown
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-aarch64-apple-darwin (default)
rustc 1.76.0 (07dca489a 2024-02-04)

Default host: aarch64-apple-darwin
rustup home:  /Users/zian/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin (default)
stable-x86_64-unknown-linux-gnu
nightly-2023-05-31-aarch64-apple-darwin
nightly-aarch64-apple-darwin

installed targets for active toolchain
--------------------------------------

aarch64-apple-darwin
wasm32-unknown-unknown

active toolchain
----------------

nightly-aarch64-apple-darwin (overridden by +toolchain on the command line)
rustc 1.78.0-nightly (3246e7951 2024-02-19)

```

## Building

```
cargo build --release
```

## Steps

[] Prepare multisig for initial supply

[] Secure sudo root key

[] mainnet.json

[] Check and verify initial supply

[] Check and verify SS58 prefix

[] Check and verify protocol ID

[] Check and verify chain ID

[] Check and verify chain name

[] Check and verify token decimals

[] Check and verify token symbol

[] Prepare runtime config of Mandala and verify everything is correct, triple check

[] Prepare and document extrinsics to be sent

[] Local test on dev machine

[] Local test on another dev machine

[] Virtual test on AWS

[] Testing auction on Rococo

[] Testing crowdloan on Rococo
