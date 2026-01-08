# spur-rs

Rust types for the [Spur Context API](https://docs.spur.us/context-api).

## Features

- Zero-copy deserialization with `serde`
- All fields are optional (no mandatory fields assumed)
- Full support for IP Context, Tag Metadata, and API Status objects
- Efficient serialization that skips `None` values

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
spur = "0.1"
serde_json = "1"
```

### Deserializing an IP Context

```rust
use spur::IpContext;

let json = r#"{
    "ip": "89.39.106.191",
    "infrastructure": "DATACENTER",
    "as": {
        "number": 49981,
        "organization": "WorldStream"
    },
    "client": {
        "behaviors": ["FILE_SHARING", "TOR_PROXY_USER"],
        "count": 4,
        "countries": 2
    }
}"#;

let context: IpContext = serde_json::from_str(json)?;
println!("IP: {:?}", context.ip);
println!("Infrastructure: {:?}", context.infrastructure);
```

### Deserializing Tag Metadata

```rust
use spur::TagMetadata;

let json = r#"{
    "tag": "OXYLABS_PROXY",
    "name": "Oxylabs",
    "isAnonymous": "true",
    "categories": ["RESIDENTIAL_PROXY", "DATACENTER_PROXY"]
}"#;

let meta: TagMetadata = serde_json::from_str(json)?;
println!("Tag: {:?}", meta.tag);
println!("Anonymous: {:?}", meta.is_anonymous);
```

### Checking API Status

```rust
use spur::ApiStatus;

let json = r#"{
    "active": true,
    "queriesRemaining": 49283,
    "serviceTier": "online"
}"#;

let status: ApiStatus = serde_json::from_str(json)?;
println!("Active: {:?}", status.active);
println!("Remaining: {:?}", status.queries_remaining);
```

## Types

### IpContext

The main IP context object with the following optional fields:

| Field | Type | Description |
|-------|------|-------------|
| `ai` | `Ai` | AI activity observed from this IP |
| `autonomous_system` | `AutonomousSystem` | BGP AS information (serialized as `as`) |
| `client` | `Client` | Client descriptive data |
| `infrastructure` | `String` | Infrastructure type |
| `ip` | `String` | IPv4/IPv6 address |
| `location` | `Location` | Geolocation data |
| `organization` | `String` | Organization assigned to the IP |
| `risks` | `Vec<String>` | Risk factors |
| `services` | `Vec<String>` | Services/protocols in use |
| `tunnels` | `Vec<Tunnel>` | Tunneling information |

### TagMetadata

Metadata and metrics for a service tag.

### ApiStatus

API token status with `active`, `queries_remaining`, and `service_tier` fields.

## License

MIT
