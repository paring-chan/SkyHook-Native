#!/bin/bash

# x86_64
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# i686
rustup target add i686-unknown-linux-gnu
rustup target add i686-pc-windows-gnu

# ARM
rustup target add aarch64-apple-darwin

# sudo pacman -S mingw-w64-binutils mingw-w64-gcc mingw-w64-headers \
#                 clang gcc zlib libmpc mpfr gmp