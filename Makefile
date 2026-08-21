.PHONY: build test clean install demo release help lint

# Default
help:
	@echo "VietLang Build & Automation System"
	@echo "==================================="
	@echo ""
	@echo "Usage:"
	@echo "  make build     Build release binary"
	@echo "  make debug     Build debug binary"
	@echo "  make test      Run all Rust unit tests & integration tests"
	@echo "  make demo      Run all curated examples, fintech suites & bootstrap compiler"
	@echo "  make install   Install binary & 55 standard modules to ~/.vietlang"
	@echo "  make release   Build release + strip binary"
	@echo "  make clean     Clean build artifacts"
	@echo "  make lint      Run cargo clippy linter"
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

# Run Curated Examples & Suites
demo: build
	@echo "=== Running all Curated VietLang Examples & Test Suites ===\n"
	@echo "--- basics/hello_world.vl ---"
	./target/release/vietlang examples/basics/hello_world.vl
	@echo "\n--- basics/data_structures.vl ---"
	./target/release/vietlang examples/basics/data_structures.vl
	@echo "\n--- basics/file_and_json.vl ---"
	./target/release/vietlang examples/basics/file_and_json.vl
	@echo "\n--- backend/sqlite_database.vl ---"
	./target/release/vietlang examples/backend/sqlite_database.vl
	@echo "\n--- fintech/vietnam_fintech.vl ---"
	./target/release/vietlang examples/fintech/vietnam_fintech.vl
	@echo "\n--- fintech/concurrency_csp.vl ---"
	./target/release/vietlang examples/fintech/concurrency_csp.vl
	@echo "\n--- full_apps/agricultural_ecommerce/tests/full_system_test.vl ---"
	./target/release/vietlang examples/full_apps/agricultural_ecommerce/tests/full_system_test.vl
	@echo "\n--- bootstrap/lexer.vl ---"
	./target/release/vietlang bootstrap/lexer.vl
	@echo "\n--- bootstrap/parser.vl ---"
	./target/release/vietlang bootstrap/parser.vl
	@echo "\nAll curated examples, fintech suites, and bootstrap tests completed successfully!"

# Install
install: build
	bash ./install.sh

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
