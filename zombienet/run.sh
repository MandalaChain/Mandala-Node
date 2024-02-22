ROOT=$(git rev-parse --show-toplevel)
CHAIN=$1
CHAIN_PATH=$ROOT/res/$CHAIN/$CHAIN.json
ZOMBIENET_PATH=zombienet

if [ -z "$1" ]; then
    echo "Usage: run.sh <chain> <zombienet-path (optional)>"
    exit 1

fi

if [ -z "$2" ]; then
    echo "Using default zombienet path: ./binaries/zombienet"
    ZOMBIENET_PATH=$ROOT/zombienet/binaries/zombienet
else
    ZOMBIENET_PATH=$2
fi

cd $ROOT
cargo build --release
rm -rf $ROOT/zombienet/binaries/mandala || true
cp $ROOT/target/release/mandala $ROOT/zombienet/binaries/

cd $ROOT/zombienet
cp $CHAIN_PATH .

PARACHAIN_WASM=$ROOT/res/$CHAIN/state/genesis-wasm.wasm
PARACHAIN_WASM=$PARACHAIN_WASM CHAIN=$CHAIN.json PATH=$ROOT/zombienet/binaries:$PATH $ZOMBIENET_PATH spawn $ROOT/zombienet/config.toml --provider native

rm $CHAIN.json
