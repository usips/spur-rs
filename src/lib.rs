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
//! ## Features
//!
//! - **Zero-copy deserialization** with serde
//! - **All fields optional** - handles partial API responses gracefully
//! - **Efficient serialization** - `None` values are omitted
//! - **Test utilities** - builders and fixtures for testing (via `test-utils` feature)
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! spur = "0.1"
//! serde_json = "1"
//! ```
//!
//! ## Usage Examples
//!
//! ### Deserializing an IP Context
//!
//! ```rust
//! use spur::IpContext;
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
//! // Check infrastructure type
//! assert_eq!(context.infrastructure.as_deref(), Some("DATACENTER"));
//!
//! // Check for VPN usage
//! let is_vpn = context.tunnels.as_ref()
//!     .map(|t| t.iter().any(|t| t.tunnel_type.as_deref() == Some("VPN")))
//!     .unwrap_or(false);
//! assert!(is_vpn);
//! ```
//!
//! ### Infrastructure Types
//!
//! | Value | Description |
//! |-------|-------------|
//! | `DATACENTER` | IP from datacenter/cloud provider |
//! | `RESIDENTIAL` | Home ISP connection |
//! | `MOBILE` | Mobile carrier network |
//! | `BUSINESS` | Business/enterprise network |
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
//! use spur::IpContext;
//!
//! fn has_risk(ctx: &IpContext, risk: &str) -> bool {
//!     ctx.risks.as_ref()
//!         .map(|risks| risks.iter().any(|r| r == risk))
//!         .unwrap_or(false)
//! }
//!
//! // Common risk values: "TUNNEL", "SPAM", "CALLBACK_PROXY", "GEO_MISMATCH"
//! ```
//!
//! ## Test Utilities
//!
//! Enable the `test-utils` feature for testing helpers:
//!
//! ```toml
//! [dev-dependencies]
//! spur = { version = "0.1", features = ["test-utils"] }
//! ```
//!
//! ```rust,ignore
//! use spur::test_utils::{IpContextBuilder, fixtures};
//!
//! // Build custom test contexts
//! let vpn_context = IpContextBuilder::new()
//!     .ip("1.2.3.4")
//!     .infrastructure("DATACENTER")
//!     .vpn("NordVPN")
//!     .add_risk("TUNNEL")
//!     .build();
//!
//! // Use pre-built fixtures
//! let residential = fixtures::residential_ip();
//! let tor_exit = fixtures::tor_exit_node();
//! let datacenter = fixtures::datacenter_proxy();
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
//!     let infra = ctx.infrastructure.as_deref().unwrap_or("unknown");
//!     let org = ctx.organization.as_deref().unwrap_or("unknown");
//!     format!("{} ({} / {})", ip, infra, org)
//! }
//! ```

mod context;
mod metadata;
mod status;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

pub use context::*;
pub use metadata::*;
pub use status::*;
