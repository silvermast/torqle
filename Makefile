SHELL=/bin/bash

build:
	npm ci
	npm run tauri build
	export TAURI_SIGNING_PRIVATE_KEY=`cat ./torqle.key`
	TARGET=x86_64-pc-windows-msvc 	make build-target
	TARGET=x86_64-unknown-linux-gnu make build-target
	TARGET=x86_64-apple-darwin 		make build-target
	TARGET=aarch64-apple-darwin 	make build-target

clean:
	rm -rf node_modules
	rm -rf src-tauri/target
	rm -rf dist

build-target:
	rustup target add $(TARGET)
	export TAURI_SIGNING_PRIVATE_KEY="$(cat ./torqle.key)" && export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="" && npm run tauri build -- --target $(TARGET)

macos-build:
	TARGET=aarch64-apple-darwin make build-target
	TARGET=x86_64-apple-darwin make build-target

linux-build:
	TARGET=x86_64-unknown-linux-gnu make build-target

list-build-results:
	@find src-tauri/target -type f -name "*.dmg"
	@find src-tauri/target -type f -name "*.deb"
	@find src-tauri/target -type f -name "*.rpm"
	@find src-tauri/target -type f -name "*.exe"

list-targets:
	@rustc --print target-list | grep -E '^(x86_64|aarch64)' | grep -E '(darwin|windows|linux)'

set-version:
	test -n $(VERSION) || (echo "VERSION is not set" && exit 1)
	
	@echo "Setting package.json version to $(VERSION)"
	cat package.json | jq '.version = "$(VERSION)"' > package.json.new
	test -n package.json.new && mv package.json.new package.json
	
	@echo "Setting tauri.conf.json version to $(VERSION)"
	cat src-tauri/tauri.conf.json | jq '.version = "$(VERSION)"' > src-tauri/tauri.conf.json.new
	test -n src-tauri/tauri.conf.json.new && mv src-tauri/tauri.conf.json.new src-tauri/tauri.conf.json
	
	@echo "Setting cargo.toml version to $(VERSION)"
	cat src-tauri/Cargo.toml | tomlq -t '.package.version = "$(VERSION)"' > src-tauri/Cargo.toml.new
	test -n src-tauri/Cargo.toml.new && mv src-tauri/Cargo.toml.new src-tauri/Cargo.toml
