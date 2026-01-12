//! # spur
//!
//! Rust types for the [Spur Context API](https://docs.spur.us/context-api).
//!
//! ## Overview
//!
//! This crate provides strongly-typed, serde-compatible data structures for
//! working with Spur's IP intelligence API. Spur provides detailed context
//! about IP addresses including VPN/proxy detection, geolocation, risk
//! assessment, and infrastructure classification.
//!
//! ## Key Types
//!
//! | Type | Purpose |
//! |------|---------|
//! | [`IpContext`] | Complete IP address intelligence (main response type) |
//! | [`Tunnel`] | VPN/proxy/Tor tunnel information |
//! | [`Location`] | Geographic location data |
//! | [`AutonomousSystem`] | BGP AS number and organization |
//! | [`Client`] | Client behavior and device information |
//! | [`TagMetadata`] | Service tag metadata and metrics |
//! | [`ApiStatus`] | API account status and quota |
//!
//! ## Strongly Typed Enums
//!
//! | Enum | Purpose |
//! |------|---------|
//! | [`Infrastructure`] | Network type (Datacenter, Residential, Mobile, Business) |
//! | [`Risk`] | Risk factors (Tunnel, Spam, CallbackProxy, GeoMismatch) |
//! | [`Service`] | Protocols (OpenVpn, Ipsec, Wireguard, Ssh) |
//! | [`TunnelType`] | Tunnel type (Vpn, Proxy, Tor) |
//! | [`Behavior`] | Client behaviors (FileSharing, TorProxyUser) |
//! | [`DeviceType`] | Device types (Mobile, Desktop) |
//!
//! All enums include an `Other(String)` variant for forward compatibility
//! with new API values.
//!
//! ## Features
//!
//! - **Strongly typed enums** with `Other(String)` fallback for extensibility
//! - **Zero-copy deserialization** with serde
//! - **All fields optional** - handles partial API responses gracefully
//! - **Efficient serialization** - `None` values are omitted
//! - **Test utilities** - builders and fixtures for testing (via `test-utils` feature)
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! spur = "0.2"
//! serde_json = "1"
//! ```
//!
//! ## Usage Examples
//!
//! ### Deserializing an IP Context
//!
//! ```rust
//! use spur::{IpContext, Infrastructure, TunnelType};
//!
//! let json = r#"{
//!     "ip": "89.39.106.191",
//!     "infrastructure": "DATACENTER",
//!     "as": { "number": 49981, "organization": "WorldStream" },
//!     "risks": ["TUNNEL", "SPAM"],
//!     "tunnels": [{ "type": "VPN", "operator": "NordVPN", "anonymous": true }]
//! }"#;
//!
//! let context: IpContext = serde_json::from_str(json).unwrap();
//!
//! // Check infrastructure type with pattern matching
//! assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));
//!
//! // Check for VPN usage
//! let is_vpn = context.tunnels.as_ref()
//!     .map(|t| t.iter().any(|t| t.tunnel_type == Some(TunnelType::Vpn)))
//!     .unwrap_or(false);
//! assert!(is_vpn);
//! ```
//!
//! ### Working with Enums
//!
//! ```rust
//! use spur::{Infrastructure, Risk};
//!
//! // Pattern matching on known variants
//! fn describe_infra(infra: &Infrastructure) -> &str {
//!     match infra {
//!         Infrastructure::Datacenter => "Cloud/Server",
//!         Infrastructure::Residential => "Home User",
//!         Infrastructure::Mobile => "Mobile Carrier",
//!         Infrastructure::Business => "Enterprise",
//!         Infrastructure::Other(s) => s.as_str(),
//!     }
//! }
//!
//! // Unknown API values deserialize to Other
//! let json = r#""SATELLITE""#;
//! let infra: Infrastructure = serde_json::from_str(json).unwrap();
//! assert!(infra.is_other());
//! ```
//!
//! ### Detecting Anonymous Traffic
//!
//! ```rust
//! use spur::IpContext;
//!
//! fn is_anonymous(ctx: &IpContext) -> bool {
//!     ctx.tunnels.as_ref()
//!         .map(|tunnels| tunnels.iter().any(|t| t.anonymous == Some(true)))
//!         .unwrap_or(false)
//! }
//! ```
//!
//! ### Checking Risk Factors
//!
//! ```rust
//! use spur::{IpContext, Risk};
//!
//! fn has_tunnel_risk(ctx: &IpContext) -> bool {
//!     ctx.risks.as_ref()
//!         .map(|risks| risks.iter().any(|r| *r == Risk::Tunnel))
//!         .unwrap_or(false)
//! }
//! ```
//!
//! ## Test Utilities
//!
//! Enable the `test-utils` feature for testing helpers:
//!
//! ```toml
//! [dev-dependencies]
//! spur = { version = "0.2", features = ["test-utils"] }
//! ```
//!
//! ```rust,ignore
//! use spur::test_utils::{IpContextBuilder, fixtures};
//!
//! // Build custom test contexts
//! let vpn_context = IpContextBuilder::new()
//!     .ip("1.2.3.4")
//!     .infrastructure(Infrastructure::Datacenter)
//!     .vpn("NordVPN")
//!     .add_risk(Risk::Tunnel)
//!     .build();
//!
//! // Use pre-built fixtures
//! let residential = fixtures::residential_ip();
//! let tor_exit = fixtures::tor_exit_node();
//! let datacenter = fixtures::datacenter_ip();
//! ```
//!
//! ## API Response Fields
//!
//! All fields in [`IpContext`] are optional since the Spur API may omit any
//! field with a null value. Use pattern matching or `.as_ref()` to safely
//! access values:
//!
//! ```rust
//! use spur::IpContext;
//!
//! fn describe(ctx: &IpContext) -> String {
//!     let ip = ctx.ip.as_deref().unwrap_or("unknown");
//!     let infra = ctx.infrastructure.as_ref()
//!         .map(|i| i.as_str())
//!         .unwrap_or("unknown");
//!     let org = ctx.organization.as_deref().unwrap_or("unknown");
//!     format!("{} ({} / {})", ip, infra, org)
//! }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]

mod context;
pub mod enums;
mod metadata;
mod status;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

#[cfg(any(test, feature = "test-utils"))]
pub mod proptest_strategies;

pub use context::*;
pub use enums::*;
pub use metadata::*;
pub use status::*;
