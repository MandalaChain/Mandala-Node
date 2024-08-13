ROOT=$(git rev-parse --show-toplevel)
CHAIN_PATH=$ROOT/res/$CHAIN/$CHAIN.json
ZOMBIENET_PATH=zombienet

if [ -z "$1" ]; then
    echo "Usage: run.sh <zombienet-path>"
    exit 1

fi

if [ -z "$1" ]; then
    echo "Using default zombienet path: ./binaries/zombienet"
    ZOMBIENET_PATH=$ROOT/zombienet/binaries/zombienet
else
    ZOMBIENET_PATH=$1
fi

PATH=$ROOT/zombienet/binaries:$PATH $ZOMBIENET_PATH spawn $ROOT/zombienet/config.toml --provider native

rm $CHAIN.json
