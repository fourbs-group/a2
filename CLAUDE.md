# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

### Building
```bash
cargo build                                  # Build with default features (ring)
cargo build --no-default-features --features ring  # Build with ring explicitly
cargo build --all-features                  # Build with all features
```

### Testing
```bash
cargo test                                   # Run tests with default features
cargo test --all-features                   # Run all tests
cargo test --no-default-features --features ring  # Test with ring feature only
cargo test test_name                        # Run a specific test
cargo test module_name::                    # Run tests in a specific module
```

### Linting and Formatting
```bash
cargo clippy -- -D clippy::all              # Run clippy linter (strict mode)
cargo clippy --no-default-features --features ring -- -D clippy::all  # Clippy with ring
cargo fmt                                    # Format code
cargo fmt -- --check                        # Check formatting without changing files
```

## Architecture Overview

### Core Modules

- **client.rs**: Main `Client` struct handling HTTP/2 connections to APNs. Manages connection pooling, authentication, and request/response handling. Supports both Production and Sandbox endpoints.

- **signer.rs**: Handles JWT token generation for token-based authentication. Manages token renewal and caching according to Apple's guidelines.

- **request/**: Request construction modules
  - **payload.rs**: Core payload structure with custom data support via Serde
  - **notification/**: Notification builders using builder pattern
    - **default.rs**: `DefaultNotificationBuilder` for standard iOS notifications
    - **web.rs**: `WebNotificationBuilder` for Safari web notifications
    - **options.rs**: Common notification options (priority, expiration, etc.)

- **response.rs**: Response parsing and error mapping from APNs

- **error.rs**: Error types covering connection, authentication, and APNs-specific errors

- **pkcs12.rs**: PKCS#12 certificate parsing using pure Rust implementation

### Authentication Patterns

The library supports two authentication methods:

1. **Certificate-based**: Uses PKCS#12 (.p12) files with password
2. **Token-based**: Uses PKCS#8 private keys (.p8) with team ID and key ID

Both create a `Client` instance that manages HTTP/2 connection pooling and reuse.

### Key Design Patterns

- **Builder Pattern**: Notification payloads use builders for type-safe construction
- **Connection Pooling**: Single `Client` instance maintains persistent HTTP/2 connections
- **Async/Await**: All operations are async using Tokio runtime
- **Feature Flags**: 
  - `ring` (default): Uses ring for cryptography
  - `tracing`: Enables tracing support for debugging

### Important Implementation Notes

- The library uses hyper with rustls for TLS connections
- HTTP/2 keep-alive is configured to send PING frames after 1 hour of inactivity (Apple's recommendation)
- Default request timeout is 20 seconds
- Responses are strongly typed with specific error reasons from APNs
- Custom notification data can be any Serde-serializable type

### Testing Approach

Tests are embedded in source files using `#[cfg(test)]` modules. Key test areas:
- Payload serialization/deserialization
- JWT token generation and validation
- Notification builder output
- Response parsing

Use `cargo test` to run all tests. Tests requiring external resources (like actual APNs connections) are typically in the examples/ directory rather than unit tests.