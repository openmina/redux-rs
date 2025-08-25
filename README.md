# Redux-RS

[![Rust 1.84](https://img.shields.io/badge/rust-1.84-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

A Rust implementation of the Redux pattern for predictable state management,
optimized for performance and type safety.

> **Note**: This is NOT a fork of
> [redux-rs/redux-rs](https://github.com/redux-rs/redux-rs). This is an
> independent implementation designed specifically for the Mina Rust node.

## Overview

Redux-RS provides a lightweight, type-safe implementation of the Redux pattern
in Rust. It enables predictable state management through unidirectional data
flow, with support for effects, services, and sub-stores. This library is part
of the [Mina Rust Node](https://github.com/o1-labs/mina-rust) implementation.

## Features

- **Type-safe Actions**: Strongly typed actions with metadata and enabling
  conditions
- **Immutable State Management**: StateWrapper ensures immutable access to state
  outside reducers
- **Effects System**: Handle side effects in a controlled manner
- **Time Services**: Built-in support for monotonic and system time
- **Sub-stores**: Compose complex state hierarchies
- **WASM Support**: Ready for WebAssembly targets
- **Fuzzing Support**: Optional fuzzing capabilities for testing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
redux = { git = "https://github.com/openmina/redux-rs.git" }
```

### Features

- `serde` (default): Serialization support
- `fuzzing`: Enable fuzzing capabilities

## Quick Start

```rust
use redux::{Store, Reducer, ActionWithMeta};

// Define your state
#[derive(Clone)]
struct AppState {
    counter: i32,
}

// Define your actions
enum AppAction {
    Increment,
    Decrement,
    Reset,
}

// Implement the reducer
fn app_reducer(state: &mut AppState, action: &ActionWithMeta<AppAction>) {
    match &action.action {
        AppAction::Increment => state.counter += 1,
        AppAction::Decrement => state.counter -= 1,
        AppAction::Reset => state.counter = 0,
    }
}

// Create and use the store
let initial_state = AppState { counter: 0 };
let mut store = Store::new(initial_state, app_reducer);

// Dispatch actions
store.dispatch(AppAction::Increment);
store.dispatch(AppAction::Increment);

// Access state immutably
assert_eq!(store.state().get().counter, 2);
```

## Core Concepts

### Actions

Actions are plain Rust enums that describe what happened. They are automatically
wrapped with metadata including timestamps and action IDs.

### Reducers

Reducers are pure functions that take the current state and an action, and
return the new state:

```rust
pub type Reducer<State, Action> = fn(&mut State, &ActionWithMeta<Action>);
```

### Store

The Store holds the application state and orchestrates the dispatch cycle:

1. Actions are dispatched to the store
2. The reducer produces a new state
3. Effects are processed
4. Subscribers are notified

### Effects

Effects handle side effects like API calls, logging, or other async operations:

```rust
use redux::Effects;

fn my_effect(store: &mut Store<State, Action>, action: &ActionWithMeta<Action>) {
    // Handle side effects here
}
```

### Sub-stores

Break down complex state into manageable pieces with sub-stores:

```rust
use redux::SubStore;

// Create sub-stores for different parts of your application
let user_store = SubStore::new(user_state, user_reducer);
let posts_store = SubStore::new(posts_state, posts_reducer);
```

## Time Services

Redux-RS includes built-in time services for handling both monotonic and system
time:

```rust
use redux::{Timestamp, SystemTime, Instant};

// Get current timestamp
let now = Timestamp::now();

// Convert between time types
let system_time: SystemTime = now.into();
```

## WASM Support

The library is designed to work in WebAssembly environments with the
`wasm-timer` dependency for WASM targets.

## Development

### Setup

```bash
make setup
```

### Building

```bash
make build        # Debug build
make release      # Release build
```

### Testing

```bash
make test         # Run unit tests
make test-all     # Run all tests including doc tests
```

### Code Quality

```bash
make check        # Run all checks (format, lint, test)
make format       # Format code
make lint         # Run linting
```

### Documentation

```bash
make generate-doc # Generate documentation
```

### Additional Targets

```bash
make build-wasm   # Build for WebAssembly
make build-fuzzing # Build with fuzzing features
make help         # Show all available targets
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for
details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
