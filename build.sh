#!/bin/bash

set -x

platform=$(uname)

echo Current platform: $platform

mkdir -p out/x86_64
mkdir -p out/aarch64
mkdir -p out/i686

if [[ $platform == 'Linux' ]]; then
  cargo build --release --target x86_64-unknown-linux-gnu
  cargo build --release --target x86_64-pc-windows-gnu
  cargo build --release --target i686-pc-windows-gnu

  cp target/x86_64-unknown-linux-gnu/release/libskyhook.so out/x86_64
  cp target/x86_64-pc-windows-gnu/release/skyhook.dll out/x86_64

  cp target/i686-pc-windows-gnu/release/skyhook.dll out/i686
elif [[ $platform == 'Darwin' ]]; then
  cargo build --release --target x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin

  cp target/x86_64-apple-darwin/release/libskyhook.dylib out/x86_64
  cp target/aarch64-apple-darwin/release/libskyhook.dylib out/aarch64
fi
