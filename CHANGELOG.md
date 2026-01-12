# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-12

### Added

- **Strongly typed enums** for all discrete API values:
  - `Infrastructure` - network type (Datacenter, Residential, Mobile, Business)
  - `Risk` - risk factors (Tunnel, Spam, CallbackProxy, GeoMismatch)
  - `Service` - protocols (OpenVpn, Ipsec, Wireguard, Ssh, Pptp)
  - `TunnelType` - tunnel type (Vpn, Proxy, Tor)
  - `Behavior` - client behaviors (FileSharing, TorProxyUser)
  - `DeviceType` - device types (Mobile, Desktop)
- All enums include `Other(String)` variant for forward compatibility
- Enum helper methods: `as_str()`, `is_other()`, `Display` implementation
- **Proptest strategies** for property-based testing (`proptest_strategies` module)
- **Integration tests** with realistic JSON fixtures
- **Crate documentation** with `#![warn(missing_docs)]`
- Code quality attributes: `#![deny(unsafe_code)]`, `#![warn(clippy::all)]`
- MSRV set to Rust 1.70
- docs.rs metadata for all-features documentation

### Changed

- **BREAKING**: `infrastructure` field type changed from `Option<String>` to `Option<Infrastructure>`
- **BREAKING**: `risks` field type changed from `Option<Vec<String>>` to `Option<Vec<Risk>>`
- **BREAKING**: `services` field type changed from `Option<Vec<String>>` to `Option<Vec<Service>>`
- **BREAKING**: `tunnel_type` field type changed from `Option<String>` to `Option<TunnelType>`
- **BREAKING**: `behaviors` field type changed from `Option<Vec<String>>` to `Option<Vec<Behavior>>`
- **BREAKING**: `types` field type changed from `Option<Vec<String>>` to `Option<Vec<DeviceType>>`
- `test-utils` feature now includes proptest as an optional dependency
- Updated `IpContextBuilder` to use typed enums instead of string parameters

### Migration Guide

Replace string comparisons with enum pattern matching:

```rust
// v0.1 (string-based)
if context.infrastructure.as_deref() == Some("DATACENTER") { ... }

// v0.2 (enum-based)
if context.infrastructure == Some(Infrastructure::Datacenter) { ... }
```

For unknown values, use the `Other` variant:

```rust
// Handle both known and unknown values
match &context.infrastructure {
    Some(Infrastructure::Datacenter) => "Cloud server",
    Some(Infrastructure::Residential) => "Home user",
    Some(Infrastructure::Other(s)) => s.as_str(),
    None => "Unknown",
}
```

## [0.1.0] - 2025-12-01

### Added

- Initial release
- `IpContext` struct for IP intelligence data
- `TagMetadata` struct for service tag metadata
- `ApiStatus` struct for API status information
- `test-utils` feature with `IpContextBuilder` and fixtures
- Full serde support with optional field handling
- Comprehensive documentation and examples
