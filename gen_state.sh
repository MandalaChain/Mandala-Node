#! bash

CHAIN=$1
CHAINS=("local" "testnet" "mainnet")

if [ -z "$1" ]; then
    echo "Usage: gen_state.sh <chain> [all, local, testnet, mainnet]"
    exit 1
fi

ROOT=$(git rev-parse --show-toplevel)

CHAIN_PATH=$ROOT/res/$CHAIN/$CHAIN.json
MANDALA_BIN=$ROOT/target/release/mandala

cd $ROOT
echo "Building mandala"
cargo build --release

gen_state() {
    CHAIN=$1
    PATH=$ROOT/res/$CHAIN/$CHAIN.json
    RAW_PATH=$ROOT/res/$CHAIN/raw-$CHAIN.json

    $MANDALA_BIN build-spec --chain $PATH --raw >$RAW_PATH
    $MANDALA_BIN export-genesis-state --chain $RAW_PATH >$ROOT/res/$CHAIN/state/genesis-state.json
    $MANDALA_BIN export-genesis-wasm --chain $RAW_PATH >$ROOT/res/$CHAIN/state/genesis-wasm.wasm
}

if [ "$1" == "all" ]; then

    for chain in "${CHAINS[@]}"; do
        echo "Generating state for $chain"
        gen_state $chain
    done
    exit 0
else
    gen_state $CHAIN

fi
