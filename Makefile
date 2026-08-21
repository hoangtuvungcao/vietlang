.PHONY: build test clean install demo release help

# Default
help:
	@echo "VietLang Build System"
	@echo "====================="
	@echo ""
	@echo "Usage:"
	@echo "  make build     Build release binary"
	@echo "  make debug     Build debug binary"
	@echo "  make test      Run all tests"
	@echo "  make demo      Run all examples and bootstrap tests"
	@echo "  make install   Install to /usr/local/bin"
	@echo "  make clean     Clean build artifacts"
	@echo "  make release   Build release + strip binary"
	@echo "  make lint      Run clippy linter"
	@echo ""

# Build
build:
	cargo build --release
	@echo "\nBinary: target/release/vietlang ($$(du -h target/release/vietlang | cut -f1))"

debug:
	cargo build

# Test
test:
	cargo test --all
	@echo "\nAll Rust unit tests passed!"

# Run examples
demo: build
	@echo "=== Running all VietLang demos ===\n"
	@echo "--- demo.vl ---"
	./target/release/vietlang examples/demo.vl
	@echo "\n--- backend_demo.vl ---"
	./target/release/vietlang examples/backend_demo.vl
	@echo "\n--- file_io.vl ---"
	./target/release/vietlang examples/file_io.vl
	@echo "\n--- json_demo.vl ---"
	./target/release/vietlang examples/json_demo.vl
	@echo "\n--- http_server.vl ---"
	./target/release/vietlang examples/http_server.vl
	@echo "\n--- database.vl ---"
	./target/release/vietlang examples/database.vl
	@echo "\n--- concurrency.vl ---"
	./target/release/vietlang examples/concurrency.vl
	@echo "\n--- new_features_test.vl ---"
	./target/release/vietlang examples/new_features_test.vl
	@echo "\n--- community_modules_demo.vl ---"
	./target/release/vietlang examples/community_modules_demo.vl
	@echo "\n--- enterprise_microservice.vl ---"
	./target/release/vietlang examples/enterprise_microservice.vl
	@echo "\n--- realtime_websocket_server.vl ---"
	./target/release/vietlang examples/realtime_websocket_server.vl
	@echo "\n--- bootstrap/parser.vl ---"
	./target/release/vietlang bootstrap/parser.vl
	@echo "\n--- bootstrap/lexer.vl ---"
	./target/release/vietlang bootstrap/lexer.vl
	@echo "\nAll demos, enterprise suites, and bootstrap tests completed successfully!"

# Install
install: build
	sudo cp target/release/vietlang /usr/local/bin/vietlang
	@echo "Installed vietlang to /usr/local/bin/"
	@vietlang --version

# Clean
clean:
	cargo clean

# Release build (optimized + stripped)
release:
	cargo build --release
	strip target/release/vietlang 2>/dev/null || true
	@echo "\nRelease binary: target/release/vietlang ($$(du -h target/release/vietlang | cut -f1))"

# Lint
lint:
	cargo clippy -- -D warnings
	@echo "No lint issues"
