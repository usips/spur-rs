# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `spur`, a Rust library providing strongly-typed, serde-compatible data structures for the [Spur Context API](https://docs.spur.us/context-api). Spur provides IP intelligence including VPN/proxy detection, geolocation, risk assessment, and infrastructure classification.

## Build & Test Commands

```bash
# Build the library
cargo build

# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run fixture tests only
cargo test fixture

# Run proptest-based fuzz tests
cargo test proptest

# Build documentation
cargo doc --open

# Check with all features
cargo check --all-features

# Clippy linting
cargo clippy --all-features
```

## Architecture

### Core Modules

- **`src/lib.rs`** - Library entry point, re-exports all public types
- **`src/context.rs`** - Main `IpContext` struct and nested types (`Tunnel`, `Location`, `Client`, etc.)
- **`src/enums.rs`** - Strongly-typed enums (`Infrastructure`, `Risk`, `TunnelType`, etc.) with `Other(String)` fallback for forward compatibility
- **`src/metadata.rs`** - `TagMetadata` type for service tag information
- **`src/status.rs`** - `ApiStatus` type for API account status

### Test Utilities (feature-gated)

- **`src/test_utils.rs`** - `IpContextBuilder` and pre-built fixtures for testing
- **`src/proptest_strategies.rs`** - Proptest strategies for property-based testing

### Key Design Patterns

1. **All fields are `Option<T>`** - Handles partial API responses gracefully
2. **Enums have `Other(String)` variants** - Forward compatibility with new API values
3. **`impl_serde_enum!` macro** - DRY implementation for enum serde traits in `enums.rs`
4. **Custom deserializer for tunnel entries** - Handles both string arrays and object arrays from the API

## Testing with Real API Data

Save JSON responses from the Spur API as fixtures:

```bash
curl -s "https://api.spur.us/v2/context/1.2.3.4" \
  -H "Token: YOUR_API_TOKEN" \
  | jq . > tests/fixtures/descriptive_name.json
```

Fixture naming convention: `vpn_*.json`, `tor_*.json`, `residential_*.json`, `datacenter_*.json`, `ai_*.json`

All fixtures in `tests/fixtures/` are automatically tested for parsing, round-trip serialization, and type-specific validation.
