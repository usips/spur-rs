# spur

[![Crates.io](https://img.shields.io/crates/v/spur.svg)](https://crates.io/crates/spur)
[![Documentation](https://docs.rs/spur/badge.svg)](https://docs.rs/spur)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Rust types for [Spur](https://spur.us) APIs including the [Context API](https://docs.spur.us/context-api) and [Monocle](https://docs.spur.us/monocle).

Spur provides IP intelligence including VPN/proxy detection, geolocation, risk assessment, and infrastructure classification. This crate offers strongly-typed, serde-compatible data structures for working with Spur API responses.

## Features

- **Multi-API support** - Context API and Monocle in one crate
- **Strongly typed enums** with `Other(String)` fallback for forward compatibility
- **Zero-copy deserialization** with serde
- **All fields optional** - handles partial API responses gracefully
- **Efficient serialization** - `None` values are omitted
- **Test utilities** - builders and fixtures for testing (via `test-utils` feature)
- **Property-based testing** - proptest strategies included

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spur = "0.3"
serde_json = "1"
```

## APIs

| Module | Purpose | Documentation |
|--------|---------|---------------|
| [`spur::context`](https://docs.rs/spur/latest/spur/context/) | IP intelligence via Context API | [docs.spur.us/context-api](https://docs.spur.us/context-api) |
| [`spur::monocle`](https://docs.rs/spur/latest/spur/monocle/) | Device-level VPN/proxy detection | [docs.spur.us/monocle](https://docs.spur.us/monocle) |

## Quick Start

### Context API

```rust
use spur::{IpContext, Infrastructure, TunnelType, Risk};

let json = r#"{
    "ip": "89.39.106.191",
    "infrastructure": "DATACENTER",
    "as": { "number": 49981, "organization": "WorldStream" },
    "risks": ["TUNNEL", "SPAM"],
    "tunnels": [{ "type": "VPN", "operator": "NordVPN", "anonymous": true }]
}"#;

let context: IpContext = serde_json::from_str(json).unwrap();

// Use strongly typed enums
assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));

// Check for VPN usage
let is_vpn = context.tunnels.as_ref()
    .map(|t| t.iter().any(|t| t.tunnel_type == Some(TunnelType::Vpn)))
    .unwrap_or(false);

// Check risk factors
let has_tunnel_risk = context.risks.as_ref()
    .map(|r| r.contains(&Risk::Tunnel))
    .unwrap_or(false);
```

### Monocle

```rust
use spur::monocle::Assessment;

// After calling the Monocle Decryption API
let json = r#"{
    "vpn": true,
    "proxied": false,
    "anon": true,
    "ip": "37.19.221.165",
    "ts": "2022-12-01T01:00:50Z",
    "complete": true,
    "id": "0a3e401a-b0d5-496b-b1ff-6cb8eca542a2",
    "sid": "checkout-form"
}"#;

let assessment: Assessment = serde_json::from_str(json).unwrap();

if assessment.is_anonymized() {
    println!("User at {} is using anonymization", assessment.ip);
}

if !assessment.is_trustworthy() {
    println!("Assessment incomplete, results may be unreliable");
}
```

## Strongly Typed Enums

All API string fields that represent discrete values are typed enums:

| Enum | Values | Field |
|------|--------|-------|
| `Infrastructure` | `Datacenter`, `Residential`, `Mobile`, `Business` | `infrastructure` |
| `Risk` | `Tunnel`, `Spam`, `CallbackProxy`, `GeoMismatch` | `risks` |
| `Service` | `OpenVpn`, `Ipsec`, `Wireguard`, `Ssh`, `Pptp` | `services` |
| `TunnelType` | `Vpn`, `Proxy`, `Tor` | `tunnel_type` |
| `Behavior` | `FileSharing`, `TorProxyUser` | `behaviors` |
| `DeviceType` | `Mobile`, `Desktop` | `types` |

All enums include an `Other(String)` variant for forward compatibility with new API values:

```rust
use spur::Infrastructure;

// Unknown values deserialize to Other
let json = r#""SATELLITE""#;
let infra: Infrastructure = serde_json::from_str(json).unwrap();
assert!(infra.is_other());
assert_eq!(infra.as_str(), "SATELLITE");
```

## Types

### Context API Types

#### IpContext

The main IP context object:

| Field | Type | Description |
|-------|------|-------------|
| `ip` | `String` | IPv4/IPv6 address |
| `infrastructure` | `Infrastructure` | Network type (datacenter, residential, etc.) |
| `organization` | `String` | Organization assigned to the IP |
| `autonomous_system` | `AutonomousSystem` | BGP AS information |
| `location` | `Location` | Geolocation data |
| `risks` | `Vec<Risk>` | Risk factors |
| `services` | `Vec<Service>` | Services/protocols in use |
| `tunnels` | `Vec<Tunnel>` | VPN/proxy/Tor information |
| `client` | `Client` | Client behavior and device data |
| `ai` | `Ai` | AI activity observed from this IP |

#### TagMetadata

Metadata and metrics for a service tag.

#### ApiStatus

API token status with `active`, `queries_remaining`, and `service_tier` fields.

### Monocle Types

#### Assessment

Decrypted assessment from the Monocle Decryption API:

| Field | Type | Description |
|-------|------|-------------|
| `vpn` | `bool` | VPN detected |
| `proxied` | `bool` | Proxy detected |
| `anon` | `bool` | Anonymous connection |
| `ip` | `String` | Detected IP address |
| `ts` | `String` | Timestamp (ISO 8601) |
| `complete` | `bool` | Assessment completed successfully |
| `id` | `String` | Unique assessment ID |
| `sid` | `String` | Session ID |

Helper methods:
- `is_anonymized()` - Returns `true` if VPN, proxy, or anonymous
- `is_trustworthy()` - Returns `true` if assessment completed

## Test Utilities

Enable the `test-utils` feature for testing helpers:

```toml
[dev-dependencies]
spur = { version = "0.3", features = ["test-utils"] }
```

### Context API Builder

```rust
use spur::test_utils::IpContextBuilder;
use spur::{Infrastructure, Risk, Service};

let context = IpContextBuilder::new()
    .ip("1.2.3.4")
    .infrastructure(Infrastructure::Datacenter)
    .asn(12345, "Example Corp")
    .vpn("NordVPN")
    .add_risk(Risk::Tunnel)
    .add_service(Service::OpenVpn)
    .build();
```

### Monocle Assessment Builder

```rust
use spur::test_utils::AssessmentBuilder;

let assessment = AssessmentBuilder::new()
    .ip("1.2.3.4")
    .vpn(true)
    .anon(true)
    .session_id("checkout")
    .build();
```

### Pre-built Fixtures

```rust
use spur::test_utils::{fixtures, monocle_fixtures};

// Context API fixtures
let residential = fixtures::residential_ip();
let vpn = fixtures::vpn_ip();
let tor = fixtures::tor_exit_node();
let datacenter = fixtures::datacenter_ip();
let ai_scraper = fixtures::ai_scraper_ip();

// Monocle fixtures
let clean = monocle_fixtures::clean_assessment();
let vpn_detected = monocle_fixtures::vpn_assessment();
let proxy_detected = monocle_fixtures::proxy_assessment();
```

### Testing with Real API Responses

Save JSON responses from the Spur API as fixtures for testing against real data:

```bash
# Save an API response as a fixture
curl -s "https://api.spur.us/v2/context/1.2.3.4" \
  -H "Token: YOUR_API_TOKEN" \
  | jq . > tests/fixtures/my_new_fixture.json
```

All JSON files in `tests/fixtures/` are automatically tested for:
- Valid parsing to `IpContext`
- Round-trip serialization
- Type-specific validation (VPN fixtures have tunnels, etc.)

See [tests/fixtures/README.md](tests/fixtures/README.md) for naming conventions and details.

### Proptest Strategies

```rust
use proptest::prelude::*;
use spur::proptest_strategies::*;

proptest! {
    #[test]
    fn context_roundtrip(context in arb_ip_context()) {
        let json = serde_json::to_string(&context).unwrap();
        let parsed: IpContext = serde_json::from_str(&json).unwrap();
        assert_eq!(context, parsed);
    }

    #[test]
    fn assessment_roundtrip(assessment in arb_assessment()) {
        let json = serde_json::to_string(&assessment).unwrap();
        let parsed: Assessment = serde_json::from_str(&json).unwrap();
        assert_eq!(assessment, parsed);
    }
}
```

## Migration from v0.2

Version 0.3.0 reorganizes the crate into modules but maintains backwards compatibility:

```rust
// Both of these work in v0.3:
use spur::IpContext;                    // Root re-export (backwards compatible)
use spur::context::IpContext;           // Explicit module path

// New Monocle support:
use spur::monocle::Assessment;
```

## Migration from v0.1

Version 0.2.0 introduced breaking changes with strongly typed enums:

```rust
// v0.1 (string-based)
assert_eq!(context.infrastructure.as_deref(), Some("DATACENTER"));

// v0.2+ (enum-based)
assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));
```

Key changes:
- `infrastructure: Option<String>` → `Option<Infrastructure>`
- `risks: Option<Vec<String>>` → `Option<Vec<Risk>>`
- `services: Option<Vec<String>>` → `Option<Vec<Service>>`
- `tunnel_type: Option<String>` → `Option<TunnelType>`
- `behaviors: Option<Vec<String>>` → `Option<Vec<Behavior>>`
- `types: Option<Vec<String>>` → `Option<Vec<DeviceType>>`

## License

MIT
