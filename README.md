# redux-rs

[![Rust 1.84](https://img.shields.io/badge/rust-1.84-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://o1-labs.github.io/redux-rs/)

A Rust implementation of the Redux pattern for predictable state management,
optimized for performance and type safety.

> **Note**: This is NOT a fork of
> [redux-rs/redux-rs](https://github.com/redux-rs/redux-rs). This is an
> independent implementation designed specifically for the o1-labs ecosystem.

## Overview

`redux-rs` provides a lightweight, type-safe implementation of the Redux pattern
in Rust. It enables predictable state management through unidirectional data
flow, with support for effects, services, and sub-stores. This library is part
of the [Mina Rust Node](https://github.com/o1-labs/mina-rust) implementation.

## Installation

```toml
[dependencies]
redux = { git = "https://github.com/o1-labs/redux-rs.git" }
```

Quick reference:

```bash
make setup        # Setup development environment
make check        # Run all checks (format, lint, test)
make help         # Show all available targets
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for
details.

## Contributing

Contributions are welcome! Please see our
**[Contributing Guide](https://o1-labs.github.io/redux-rs/contributing/)** for
details.
