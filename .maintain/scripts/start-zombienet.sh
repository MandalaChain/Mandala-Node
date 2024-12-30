#!/bin/bash

ROOT=$(git rev-parse --show-toplevel)
CHAIN_TYPE=$1
ZOMBIENET_PATH=$ROOT/.maintain/zombienet/binaries/zombienet

if [ -z "$1" ]; then
    echo "Usage: ./start-zombienet.sh <chain-type> [zombienet-path]"
    echo "  <chain-type>: local, mainnet, dev, or live"
    echo "  [zombienet-path]: Optional path to zombienet binary (default: $ZOMBIENET_PATH)"
    exit 1
fi

if [ -z "$2" ]; then
    echo "Using default zombienet path: $ZOMBIENET_PATH"
else
    ZOMBIENET_PATH=$2
fi

case $CHAIN_TYPE in
    dev|live)
        CHAIN_DIR="niskala"
        ;;
    local|mainnet)
        CHAIN_DIR="mandala"
        ;;
    *)
        echo "Invalid chain type. Please use local, mainnet, dev, or live."
        exit 1
        ;;
esac

CHAIN_PATH=$ROOT/res/$CHAIN_DIR/$CHAIN_TYPE/$CHAIN_TYPE.json

cd $ROOT

cargo update

case $CHAIN_TYPE in
    dev|live)
        cargo build --release --features niskala-native
        ;;
    local|mainnet)
        cargo build --release --features mandala-native
        ;;
esac

# Make mandala binary executable and copy to binaries directory
chmod +x ./target/release/mandala
mkdir -p $ROOT/.maintain/zombienet/binaries
cp ./target/release/mandala $ROOT/.maintain/zombienet/binaries/

./target/release/mandala build-spec --chain dev > $CHAIN_PATH

cd $ROOT/.maintain/zombienet
cp $CHAIN_PATH .

PATH=$ROOT/.maintain/zombienet/binaries:$PATH CHAIN=$CHAIN_TYPE.json $ZOMBIENET_PATH spawn $ROOT/zombienet/config.toml --provider native

rm $CHAIN_TYPE.json
