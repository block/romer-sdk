#!/bin/bash
BINDINGS_DIR="./bindings/swift"
UNIFFI_BINDGEN_BIN="cargo run --manifest-path bindings/uniffi-bindgen/Cargo.toml"

cargo build --release || exit 1
$UNIFFI_BINDGEN_BIN generate bindings/romer.udl --language swift -o "$BINDINGS_DIR" || exit 1

mkdir -p $BINDINGS_DIR

# Install rust target toolchains
rustup install 1.73.0
rustup component add rust-src --toolchain 1.73.0
rustup target add aarch64-apple-ios x86_64-apple-ios --toolchain 1.73.0
rustup target add aarch64-apple-ios-sim --toolchain 1.73.0
rustup target add aarch64-apple-darwin x86_64-apple-darwin --toolchain 1.73.0

# Build rust target libs
cargo build --profile release-smaller --features uniffi || exit 1
cargo build --profile release-smaller --features uniffi --target x86_64-apple-darwin || exit 1
cargo build --profile release-smaller --features uniffi --target aarch64-apple-darwin || exit 1
cargo build --profile release-smaller --features uniffi --target x86_64-apple-ios || exit 1
cargo build --profile release-smaller --features uniffi --target aarch64-apple-ios || exit 1
cargo +1.73.0 build --release --features uniffi --target aarch64-apple-ios-sim || exit 1

# Combine ios-sim and apple-darwin (macos) libs for x86_64 and aarch64 (m1)
mkdir -p target/lipo-ios-sim/release-smaller || exit 1
lipo target/aarch64-apple-ios-sim/release/libromer.a target/x86_64-apple-ios/release-smaller/libromer.a -create -output target/lipo-ios-sim/release-smaller/libromer.a || exit 1
mkdir -p target/lipo-macos/release-smaller || exit 1
lipo target/aarch64-apple-darwin/release-smaller/libromer.a target/x86_64-apple-darwin/release-smaller/libromer.a -create -output target/lipo-macos/release-smaller/libromer.a || exit 1

$UNIFFI_BINDGEN_BIN generate bindings/romer.udl --language swift -o "$BINDINGS_DIR" || exit 1

swiftc -module-name Romer -emit-library -o "$BINDINGS_DIR"/libromer.dylib -emit-module -emit-module-path "$BINDINGS_DIR" -parse-as-library -L ./target/release-smaller -lromer -Xcc -fmodule-map-file="$BINDINGS_DIR"/RomerFFI.modulemap "$BINDINGS_DIR"/Romer.swift -v || exit 1

# Create xcframework from bindings Swift file and libs
mkdir -p "$BINDINGS_DIR"/Sources/Romer || exit 1

# Patch Romer.swift with `SystemConfiguration` import.
sed -i '' '4s/^/import SystemConfiguration\n/' "$BINDINGS_DIR"/Romer.swift

mv "$BINDINGS_DIR"/Romer.swift "$BINDINGS_DIR"/Sources/Romer/Romer.swift || exit 1
cp "$BINDINGS_DIR"/RomerFFI.h "$BINDINGS_DIR"/RomerFFI.xcframework/ios-arm64/RomerFFI.framework/Headers || exit 1
cp "$BINDINGS_DIR"/RomerFFI.h "$BINDINGS_DIR"/RomerFFI.xcframework/ios-arm64_x86_64-simulator/RomerFFI.framework/Headers || exit 1
cp "$BINDINGS_DIR"/RomerFFI.h "$BINDINGS_DIR"/RomerFFI.xcframework/macos-arm64_x86_64/RomerFFI.framework/Headers || exit 1
cp target/aarch64-apple-ios/release-smaller/libromer.a "$BINDINGS_DIR"/RomerFFI.xcframework/ios-arm64/RomerFFI.framework/RomerFFI || exit 1
cp target/lipo-ios-sim/release-smaller/libromer.a "$BINDINGS_DIR"/RomerFFI.xcframework/ios-arm64_x86_64-simulator/RomerFFI.framework/RomerFFI || exit 1
cp target/lipo-macos/release-smaller/libromer.a "$BINDINGS_DIR"/RomerFFI.xcframework/macos-arm64_x86_64/RomerFFI.framework/RomerFFI || exit 1
rm "$BINDINGS_DIR"/RomerFFI.h || exit 1
rm "$BINDINGS_DIR"/RomerFFI.modulemap || exit 1
echo finished successfully!
