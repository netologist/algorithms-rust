.PHONY: build check test lint fmt clean docs list help

# ── Default target ─────────────────────────────────────────────────────────────
.DEFAULT_GOAL := help

# ── Build ──────────────────────────────────────────────────────────────────────
build:  ## Build all workspace crates (release)
	cargo build --release

check:  ## Check all workspace crates (fast, no codegen)
	cargo check --workspace

# ── Test ───────────────────────────────────────────────────────────────────────
test:  ## Run all workspace tests
	cargo test --workspace

test-summary:  ## Run all tests with concise output
	cargo test --workspace -- --format=terse

# ── Lint & Format ──────────────────────────────────────────────────────────────
lint:  ## Run clippy on all crates
	cargo clippy --workspace -- -D warnings

fmt:  ## Check formatting (CI mode)
	cargo fmt --all -- --check

fmt-fix:  ## Apply formatting
	cargo fmt --all

# ── Clean ──────────────────────────────────────────────────────────────────────
clean:  ## Remove build artifacts
	cargo clean

clean-lock:  ## Rebuild lockfile
	rm -f Cargo.lock && cargo check

# ── Documentation ──────────────────────────────────────────────────────────────
docs:  ## Generate rustdoc for all crates
	cargo doc --workspace --no-deps --open

# ── List ───────────────────────────────────────────────────────────────────────
list:  ## List all pattern crates
	@find patterns -mindepth 2 -maxdepth 2 -name Cargo.toml \
		-exec sh -c 'grep "^name" {} | head -1' \; | \
		sed 's/name\s*=\s*//' | tr -d '"' | sort

# ── Help ───────────────────────────────────────────────────────────────────────
help:  ## Show this help
	@echo "usage: make [target]"
	@echo ""
	@echo "targets:"
	@grep -E '^[a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'
