#!/bin/bash
PROJNAME=spin2win

if cargo --version | grep -q "nightly"; then
	CARGO_CMD="cargo"
else
	CARGO_CMD="cargo +nightly"
fi

$CARGO_CMD build --release --features generate-api-description --target=wasm32-unknown-unknown --verbose
# wasm2wat -o target/$PROJNAME.wat target/wasm32-unknown-unknown/release/$PROJNAME.wasm
# cat target/$PROJNAME.wat | sed "s/(import \"env\" \"memory\" (memory (;0;) 2))/(import \"env\" \"memory\" (memory (;0;) 2 16))/" > target/$PROJNAME-fixed.wat
# wat2wasm -o target/$PROJNAME.wasm target/$PROJNAME-fixed.wat
# wasm-opt -Oz target/$PROJNAME.wasm -o target/$PROJNAME-opt.wasm
# wasm-prune --exports call,deploy target/$PROJNAME-opt.wasm target/$PROJNAME-pruned.wasm
wasm-build target spin2win --target-runtime=substrate --final=spin2win --save-raw=./target/spin2win-deployed.wasm --target wasm32-unknown-unknown
