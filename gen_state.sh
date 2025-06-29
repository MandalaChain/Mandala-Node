#! bash

CHAIN=$1
CHAINS=("local" "mainnet" "dev" "live")

if [ -z "$1" ]; then
    echo "Usage: gen_state.sh <chain> [all, local, mainnet, dev, live]"
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
    
    # Determine the correct directory structure
    if [[ "$CHAIN" == "local" ]] || [[ "$CHAIN" == "mainnet" ]]; then
        CHAIN_DIR="mandala"
    elif [[ "$CHAIN" == "dev" ]] || [[ "$CHAIN" == "live" ]]; then
        CHAIN_DIR="niskala"
    else
        echo "Unknown chain: $CHAIN"
        exit 1
    fi
    
    PATH=$ROOT/res/$CHAIN_DIR/$CHAIN/$CHAIN.json
    RAW_PATH=$ROOT/res/$CHAIN_DIR/$CHAIN/$CHAIN-raw.json

    $MANDALA_BIN build-spec --chain $PATH --raw >$RAW_PATH
    $MANDALA_BIN export-genesis-state --chain $RAW_PATH >$ROOT/res/$CHAIN_DIR/$CHAIN/state/genesis-state
    $MANDALA_BIN export-genesis-wasm --chain $RAW_PATH >$ROOT/res/$CHAIN_DIR/$CHAIN/state/genesis-wasm.wasm
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
