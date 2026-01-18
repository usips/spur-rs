//! # Context API Types
//!
//! This module contains types for the [Spur Context API](https://docs.spur.us/context-api),
//! which provides detailed IP intelligence including VPN/proxy detection, geolocation,
//! risk assessment, and infrastructure classification.
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
//! ## Example
//!
//! ```rust
//! use spur::context::{IpContext, Infrastructure, TunnelType};
//!
//! let json = r#"{
//!     "ip": "89.39.106.191",
//!     "infrastructure": "DATACENTER",
//!     "tunnels": [{ "type": "VPN", "operator": "NordVPN" }]
//! }"#;
//!
//! let context: IpContext = serde_json::from_str(json).unwrap();
//! assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));
//! ```

mod enums;
mod metadata;
mod status;
mod types;

pub use enums::*;
pub use metadata::*;
pub use status::*;
pub use types::*;
