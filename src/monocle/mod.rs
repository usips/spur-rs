//! # Monocle API Types
//!
//! This module contains types for the [Spur Monocle API](https://docs.spur.us/monocle),
//! a lightweight, captcha-like JavaScript utility that passively identifies traffic
//! from commercial VPNs, anonymizing datacenter proxies, and residential proxies.
//!
//! ## Overview
//!
//! Monocle works by:
//! 1. Embedding a JavaScript snippet on your pages
//! 2. Collecting an encrypted assessment on the client side
//! 3. Decrypting the assessment via the Decryption API or your private key
//!
//! ## Key Types
//!
//! | Type | Purpose |
//! |------|---------|
//! | [`Assessment`] | Decrypted assessment result with VPN/proxy detection |
//!
//! ## Example
//!
//! ```rust
//! use spur::monocle::Assessment;
//!
//! // After calling the Monocle Decryption API:
//! let json = r#"{
//!     "vpn": true,
//!     "proxied": false,
//!     "anon": true,
//!     "ip": "37.19.221.165",
//!     "ts": "2022-12-01T01:00:50Z",
//!     "complete": true,
//!     "id": "0a3e401a-b0d5-496b-b1ff-6cb8eca542a2",
//!     "sid": "example-form"
//! }"#;
//!
//! let assessment: Assessment = serde_json::from_str(json).unwrap();
//! assert!(assessment.vpn);
//! assert!(assessment.anon);
//! ```
//!
//! ## API Endpoint
//!
//! The Monocle Decryption API endpoint is:
//! ```text
//! POST https://decrypt.mcl.spur.us/api/v1/assessment
//! Content-Type: text/plain; charset=utf-8
//! TOKEN: <MONOCLE_SECRET_KEY>
//!
//! <encrypted_bundle>
//! ```

mod types;

pub use types::*;
