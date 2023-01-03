.PHONY: build thread www
build:
	cargo build --target wasm32-wasi
	wasm-snip ./target/wasm32-wasi/debug/game_logic.wasm -o ./target/wasm32-wasi/debug/game_logic1.wasm -p wbg
	wasm-snip ./target/wasm32-wasi/debug/game_logic1.wasm -o ./target/wasm32-wasi/debug/game_logic.wasm -p bindgen
	wasmedge ./target/wasm32-wasi/debug/game_logic.wasm
thread:
	cargo build --target wasm32-wasi
	wasm-snip ./target/wasm32-wasi/debug/thread.wasm -o ./target/wasm32-wasi/debug/thread1.wasm -p wbg
	wasm-snip ./target/wasm32-wasi/debug/thread1.wasm -o ./target/wasm32-wasi/debug/thread.wasm -p bindgen
	wasmedge ./target/wasm32-wasi/debug/thread.wasm
www:
	(cd www && cargo make wasm-serve)