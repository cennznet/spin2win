# Spin2Win Contract
A smart contract which transfers a random amount of balance
to the calling account.

## Setup Environment
```bash
# osx
brew install wabt
cargo install pwasm-utils-cli --bin wasm-build
```

## Create contract wasm
```bash
# Outputs wasm binary at `./target/spin2win.wasm`
./build.sh
```
