clean:
	rm static/webview_yew_minimal_frontend.js
	rm static/webview_yew_minimal_frontend.wasm

process:
	cd frontend && cargo web build --release
	cp frontend/target/wasm32-unknown-unknown/release/webview_yew_minimal_frontend.js static/
	cp frontend/target/wasm32-unknown-unknown/release/webview_yew_minimal_frontend.wasm static/

build:
	cargo build

run:
	cargo run