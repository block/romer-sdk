#!/bin/bash
BINDINGS_DIR="./bindings/python/src/romer"
UNIFFI_BINDGEN_BIN="cargo run --manifest-path bindings/uniffi-bindgen/Cargo.toml"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	DYNAMIC_LIB_PATH="./target/release-smaller/libromer.so"
else
	DYNAMIC_LIB_PATH="./target/release-smaller/libromer.dylib"
fi

cargo build --profile release-smaller --features uniffi || exit 1
$UNIFFI_BINDGEN_BIN generate bindings/romer.udl --language python -o "$BINDINGS_DIR" || exit 1

mkdir -p $BINDINGS_DIR
cp "$DYNAMIC_LIB_PATH" "$BINDINGS_DIR" || exit 1
