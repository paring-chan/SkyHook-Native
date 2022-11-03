#!/bin/bash

set -x

mkdir -p out/x86_64
mkdir -p out/i686

# x86_64
cargo build --release --target x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-apple-darwin
echo [Windows]
cargo build --release --target x86_64-pc-windows-gnu


# i686
cargo build --release --target i686-unknown-linux-gnu
cargo build --release --target i686-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/libskyhook.so out/x86_64
cp target/x86_64-pc-windows-gnu/release/skyhook.dll out/x86_64

cp target/i686-unknown-linux-gnu/release/libskyhook.so out/i686
cp target/i686-pc-windows-gnu/release/skyhook.dll out/i686
