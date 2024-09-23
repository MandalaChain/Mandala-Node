ROOT=$(git rev-parse --show-toplevel)
CHAIN=$1
CHAIN_PATH=$ROOT/res/$CHAIN/$CHAIN.json
ZOMBIENET_PATH=zombienet

if [ -z "$1" ]; then
    echo "Usage: run.sh <chain> <zombienet-path>"
    exit 1

fi

if [ -z "$2" ]; then
    echo "Using default zombienet path: ./binaries/zombienet"
    ZOMBIENET_PATH=$ROOT/zombienet/binaries/zombienet
else
    ZOMBIENET_PATH=$2
fi

cargo update
cargo build --release --features niskala-native
cd $ROOT
./target/release/mandala build-spec --chain dev > $CHAIN_PATH

cd $ROOT/zombienet
cp $CHAIN_PATH .

PATH=$ROOT/zombienet/binaries:$PATH CHAIN=$CHAIN.json $ZOMBIENET_PATH spawn $ROOT/zombienet/config.toml --provider native

rm $CHAIN.json
