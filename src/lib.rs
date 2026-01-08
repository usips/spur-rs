//! Spur Context API types for Rust
//!
//! This crate provides serde-compatible types for the [Spur Context API](https://docs.spur.us/context-api).
//! All fields are optional since the API may omit any field if its value is null.
//!
//! # Example
//!
//! ```rust
//! use spur::IpContext;
//!
//! let json = r#"{
//!     "ip": "89.39.106.191",
//!     "infrastructure": "DATACENTER",
//!     "as": {
//!         "number": 49981,
//!         "organization": "WorldStream"
//!     }
//! }"#;
//!
//! let context: IpContext = serde_json::from_str(json).unwrap();
//! assert_eq!(context.ip.as_deref(), Some("89.39.106.191"));
//! ```

mod context;
mod metadata;
mod status;

pub use context::*;
pub use metadata::*;
pub use status::*;
