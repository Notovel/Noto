cb := cargo build

webapp:
	@cd vel-web
	wasm-pack build --target web --out-dir pkg
	@cd ..

web: webapp

