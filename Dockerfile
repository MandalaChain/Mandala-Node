# Start with a Rust image for building
FROM rust:latest

# Set working directory
WORKDIR /app

# Install protobuf-compiler for compiling Protocol Buffers
RUN apt-get update && apt-get install -y bash protobuf-compiler libprotobuf-dev clang libclang-dev

RUN rustup target add wasm32-unknown-unknown --toolchain 1.82.0

RUN rustup component add rust-src --toolchain 1.82.0

# Copy the current project into the container
COPY . .

# Build the project with the specified features and flags
RUN cargo build --release --features niskala-native

RUN cp target/release/mandala zombienet/binaries

RUN rm -rf target/

RUN .maintain/scripts/compile-mandala-polkadot.sh

RUN .maintain/scripts/download-zombienet.sh

# Generate the required chain specification and export genesis state/wasm
RUN mkdir -p res/dev/state

RUN ./zombienet/binaries/mandala build-spec --chain dev > res/dev/dev.json && \
    ./zombienet/binaries/mandala export-genesis-state --chain res/dev/dev.json > res/dev/state/genesis-state.json && \
    ./zombienet/binaries/mandala export-genesis-wasm --chain res/dev/dev.json > res/dev/state/genesis-wasm.wasm

WORKDIR /app/zombienet

# Set the entrypoint to the zombienet script with the dev chain
ENTRYPOINT ["./run.sh", "dev", "./binaries/zombienet"]
