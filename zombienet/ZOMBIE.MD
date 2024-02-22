## Zombienet
1. copy all the polkadot binaries from https://github.com/paritytech/polkadot/releases to the binaries folder
2. copy the zombienet binary from https://github.com/paritytech/zombienet/releases to the binaries folder
3. To run the zombienet script, go to the zombienet directory

```bash
cd zombienet
```

and then, run the script
```bash
./run.sh <chain> <zombienet-path (optional)>
```

so if you wanted to run the zombienet with local chainsopec with custom zombienet path, you can do
```bash
./run.sh local path-to-zombienet
```