.PHONY: build test clean install demo release help lint fuzz load soak

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
	@echo "  make fuzz      Run deterministic mutation fuzzing"
	@echo "  make load      Run bounded HTTP load test (server must be running)"
	@echo "  make soak      Run one-hour HTTP soak test (server must be running)"
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
	cargo run -- tests/p0_security.vl
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
	@echo "\n--- fintech/concurrency_csp.vl ---"
	./target/release/vietlang examples/fintech/concurrency_csp.vl
	@echo "\n--- bootstrap/lexer.vl ---"
	./target/release/vietlang bootstrap/lexer.vl
	@echo "\n--- bootstrap/parser.vl ---"
	./target/release/vietlang bootstrap/parser.vl
	@echo "\nAll curated core examples and bootstrap tests completed successfully!"

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

fuzz:
	cargo run -- fuzz --iterations 100000 --seed 1447642452

load:
	VIETLANG_LOAD_REQUESTS=10000 VIETLANG_LOAD_CONCURRENCY=64 scripts/load_soak.sh

soak:
	VIETLANG_LOAD_REQUESTS=1000 VIETLANG_LOAD_CONCURRENCY=64 VIETLANG_SOAK_SECONDS=3600 scripts/load_soak.sh
