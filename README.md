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

### For MacOS Users
When you see error like this:

```
cargo:warning=xcrun: error: unable to lookup item 'PlatformVersion' from command line tools installation
cargo:warning=xcrun: error: unable to lookup item 'PlatformVersion' in SDK '/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk'
```

The workaround is to set `MACOSX_DEPLOYMENT_TARGET` to your OS version.

For example, to set it to 14.6.1:
```
export MACOSX_DEPLOYMENT_TARGET=14.6.1
```

and then run `cargo build` again. 

Reference: https://github.com/rust-lang/cc-rs/issues/1001

## Steps

[ x ] Prepare multisig for initial supply

[ x ] Secure sudo root key

[ x ] mainnet.json

[ x ] Check and verify initial supply (1B KPGT)

[ x ] Check and verify SS58 prefix

[ x ] Check and verify protocol ID

[ x ] Check and verify chain ID

[ x ] Check and verify chain name

[ x ] Check and verify token decimals

[ x ] Check and verify token symbol

[] Prepare runtime config of Mandala and verify everything is correct, triple check

[] Prepare and document extrinsics to be sent

[ x ] Local test on dev machine (zian)

[] Local test on another dev machine

[] Virtual test on AWS

[] Testing auction on Rococo

[] Testing crowdloan on Rococo


## Notes
Probably setup a workflow job to automatically update state if any chain spec is changed