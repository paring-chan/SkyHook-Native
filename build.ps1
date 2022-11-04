mkdir out/x86_64 -ErrorAction SilentlyContinue
mkdir out/i686 -ErrorAction SilentlyContinue

cargo build --release --target=x86_64-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc

Copy-Item target/x86_64-pc-windows-msvc/release/skyhook.dll out/x86_64
Copy-Item target/i686-pc-windows-msvc/release/skyhook.dll out/i686
