# Claude Development Notes

This file contains development notes and commands for working with this project
using Claude Code.

## Quick Commands

### Development Setup

```bash
make setup
```

### Building and Testing

```bash
make build        # Debug build
make release      # Release build
make test-all     # Run all tests
make check        # Run all checks (format, lint, test)
```

### Code Quality

```bash
make format       # Format Rust code
make format-md    # Format markdown files
make lint         # Run clippy linting
make check-format # Check code formatting
make check-md     # Check markdown formatting
make check-trailing-whitespace # Check for trailing whitespace
```

### Documentation

```bash
make generate-doc # Generate documentation
```

### Specialized Builds

```bash
make build-wasm   # Build for WebAssembly
make build-fuzzing # Build with fuzzing features
```

### Utilities

```bash
make fix-trailing-whitespace # Remove trailing whitespace
make clean        # Clean build artifacts
make help         # Show all available targets
```

## Project Structure

- `src/` - Main Rust source code
  - `action/` - Action types and metadata
  - `lib.rs` - Main library entry point
  - `store.rs` - Core store implementation
  - `reducer.rs` - Reducer trait and types
  - `effects.rs` - Side effects handling
  - `service.rs` - Service interfaces
  - `timestamp.rs` - Time handling utilities
- `Cargo.toml` - Rust package configuration
- `Makefile` - Build automation
- `.github/workflows/ci.yml` - GitHub Actions CI/CD
- `.rustfmt.toml` - Rust formatting configuration
- `package.json` - Node.js dependencies (for prettier)

## CI/CD Information

The project uses GitHub Actions with the following jobs:

- **check**: Code formatting, linting, and markdown checks
- **test**: Multi-OS testing (Ubuntu, macOS) with Rust 1.84, beta, nightly
- **test-features**: Feature flag testing
- **wasm**: WebAssembly build verification
- **coverage**: Code coverage analysis
- **docs**: Documentation generation

### Supported Platforms

- Ubuntu latest, Ubuntu 24.04, macOS latest
- ARM64 support for stable builds
- WebAssembly target support

## Dependencies

### Rust Dependencies

- `serde` (optional, default) - Serialization support
- `enum_dispatch` - Efficient enum dispatch
- `fuzzcheck` (optional) - Fuzzing capabilities
- `wasm-timer` (WASM targets) - Timer support for WebAssembly

### Development Tools

- Rust 1.84 (primary toolchain)
- Nightly Rust (for formatting)
- prettier (for markdown formatting)
- cargo-tarpaulin (for coverage)

## Notes

- This is part of the [Mina Rust Node](https://github.com/o1-labs/mina-rust)
  implementation
- Uses composite actions from mina-rust for consistent CI setup
- Formatting uses nightly Rust for advanced features
- Supports both native and WebAssembly compilation
