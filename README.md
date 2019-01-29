# Spin 2 Win Contract
A smart contract which transfers a random amount of balance
to the calling account.

## Setup Environemnt
```bash
# osx
brew install wabt
cargo install pwasm-utils-cli --bin wasm-build --force
```

## Create contract wasm
```bash
./build.sh # dumps to `./target/spin2win.wasm`
```
