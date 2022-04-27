wasm:
	wasm-pack build --no-typescript

build: wasm
	cd www && npm i . 

serve:
	wasm-pack build --profiling --no-typescript
	cd www && npm run start

test:
	cargo test

package: wasm
	rm -rf docs
	cd www && npm run build
