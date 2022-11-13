#!/bin/bash

set -x

platform=$(uname)

echo Current platform: $platform

if [[ $platform == 'Linux' ]]; then
  mkdir -p out/x86_64
  mkdir -p out/i686

  cargo build --release --target x86_64-unknown-linux-gnu

  cp target/x86_64-unknown-linux-gnu/release/libskyhook.so out/x86_64
elif [[ $platform == 'Darwin' ]]; then
  TARGET_X86=x86_64-apple-darwin
  TARGET_ARM=aarch64-apple-darwin

  cargo build --release --target $TARGET_X86
  cargo build --release --target $TARGET_ARM

  lipo -create -output out/skyhook.bundle \
    target/${TARGET_X86}/release/libskyhook.dylib \
    target/${TARGET_ARM}/release/libskyhook.dylib

  # cp target/x86_64-apple-darwin/release/libskyhook.dylib out/x86_64
  # cp target/aarch64-apple-darwin/release/libskyhook.dylib out/aarch64
fi
