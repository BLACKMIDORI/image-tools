wasm:
	cd rust_core; cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen ./rust_core/target/wasm32-unknown-unknown/release/rust_core.wasm --out-dir ./static/wasm --target no-modules
