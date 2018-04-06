all: rust webpack html

rust:
	cargo +nightly build --target wasm32-unknown-unknown 
	wasm-bindgen target/wasm32-unknown-unknown/debug/signify_web.wasm --out-dir .

webpack:
	npx webpack

html:
	cp index.html *.css dist/
