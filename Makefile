wasm:
	wasm-pack build

build: wasm
	cd www && npm i . 

serve: build
	cd www && npm run start

test:
	cargo test
