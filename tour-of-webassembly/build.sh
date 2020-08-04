cargo clean
cargo build --target=wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/tour_of_webassembly.wasm ./main.wasm
