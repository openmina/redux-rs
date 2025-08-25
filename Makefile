# Redux-RS Makefile
# Based on o1-labs/proof-systems patterns

# Default target
.PHONY: all
all: build ## Build the project (default)

# Setup development environment
.PHONY: setup
setup: ## Setup development environment
	@echo "Setting up development environment..."
	@rustup install 1.84
	@rustup default 1.84
	@rustup component add clippy rustfmt
	@rustup toolchain install nightly --component rustfmt

# Build the project
.PHONY: build
build: ## Build the project
	@echo "Building redux-rs..."
	@cargo build --all-targets

# Build in release mode
.PHONY: release
release: ## Build in release mode
	@echo "Building redux-rs in release mode..."
	@cargo build --release --all-targets

# Clean build artifacts
.PHONY: clean
clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	@cargo clean

# Run tests
.PHONY: test
test: ## Run tests
	@echo "Running tests..."
	@cargo test

# Run documentation tests
.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "Running documentation tests..."
	@cargo test --doc

# Run all tests including documentation tests
.PHONY: test-all
test-all: test test-doc ## Run all tests
	@echo "All tests completed"

# Run tests with coverage (requires cargo-tarpaulin)
.PHONY: test-with-coverage
test-with-coverage: ## Run tests with coverage
	@echo "Running tests with coverage..."
	@cargo tarpaulin --out Html --out Lcov

# Check code formatting
.PHONY: check-format
check-format: ## Check code formatting
	@echo "Checking code formatting..."
	@cargo +nightly fmt --all -- --check

# Format code
.PHONY: format
format: ## Format code
	@echo "Formatting code..."
	@cargo +nightly fmt --all

# Format markdown files
.PHONY: format-md
format-md: ## Format all markdown files to wrap at 80 characters
	@echo "Formatting markdown files..."
	@npx prettier --write "**/*.md"
	@echo "Markdown files have been formatted to 80 characters."

# Check markdown formatting
.PHONY: check-md
check-md: ## Check if markdown files are properly formatted
	@echo "Checking markdown formatting..."
	@npx prettier --check "**/*.md"
	@echo "Markdown format check completed."

# Fix trailing whitespace
.PHONY: fix-trailing-whitespace
fix-trailing-whitespace: ## Remove trailing whitespaces from all files
	@echo "Removing trailing whitespaces from all files..."
	@find . -type f \( \
		-name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.yaml" \
		-o -name "*.yml" -o -name "*.json" -o -name "*.sh" \) \
		-not -path "./target/*" \
		-not -path "./node_modules/*" \
		-not -path "./.git/*" \
		-exec sed -i'' -e "s/[[:space:]]*$$//" {} + && \
		echo "Trailing whitespaces removed."

# Check trailing whitespace
.PHONY: check-trailing-whitespace
check-trailing-whitespace: ## Check for trailing whitespaces in source files
	@echo "Checking for trailing whitespaces..."
	@files_with_trailing_ws=$$(find . -type f \( \
		-name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.yaml" \
		-o -name "*.yml" -o -name "*.json" -o -name "*.sh" \) \
		-not -path "./target/*" \
		-not -path "./node_modules/*" \
		-not -path "./.git/*" \
		-exec grep -l '[[:space:]]$$' {} + 2>/dev/null || true); \
	if [ -n "$$files_with_trailing_ws" ]; then \
		echo "❌ Files with trailing whitespaces found:"; \
		echo "$$files_with_trailing_ws" | sed 's/^/  /'; \
		echo ""; \
		echo "Run 'make fix-trailing-whitespace' to fix automatically."; \
		exit 1; \
	else \
		echo "✅ No trailing whitespaces found."; \
	fi

# Run linting
.PHONY: lint
lint: ## Run clippy linting
	@echo "Running clippy linting..."
	@cargo clippy --all-targets -- -D warnings

# Check everything (format, lint, test)
.PHONY: check
check: check-format check-md check-trailing-whitespace lint test-all ## Run all checks
	@echo "All checks passed"

# Generate documentation
.PHONY: generate-doc
generate-doc: ## Generate documentation
	@echo "Generating documentation..."
	@RUSTDOCFLAGS="-D warnings --enable-index-page -Zunstable-options" cargo +nightly doc --features serde,serializable_callbacks --no-deps --workspace
	@echo ""
	@echo "The documentation is available at: ./target/doc"
	@echo ""

# Build for WASM target
.PHONY: build-wasm
build-wasm: ## Build for WASM target
	@echo "Building for WASM target..."
	@cargo build --target wasm32-unknown-unknown

# Build with fuzzing features
.PHONY: build-fuzzing
build-fuzzing: ## Build with fuzzing features (currently broken due to fuzzcheck dependency)
	@echo "Building with fuzzing features..."
	@echo "WARNING: Fuzzing is currently broken due to fuzzcheck dependency compatibility issues"
	@cargo +nightly build --features fuzzing

# Run fuzzing tests (if available)
.PHONY: test-fuzzing
test-fuzzing: ## Run fuzzing tests (currently broken due to fuzzcheck dependency)
	@echo "Running fuzzing tests..."
	@echo "WARNING: Fuzzing is currently broken due to fuzzcheck dependency compatibility issues"
	@cargo +nightly test --features fuzzing

# Help target
.PHONY: help
help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'