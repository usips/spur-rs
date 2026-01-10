//! # Test Utilities for Spur API Types
//!
//! This module provides builders, fixtures, and helper functions for testing
//! code that uses the Spur Context API types.
//!
//! ## Features
//!
//! Enable this module by adding `test-utils` feature:
//!
//! ```toml
//! [dev-dependencies]
//! spur = { version = "0.1", features = ["test-utils"] }
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use spur::test_utils::{IpContextBuilder, fixtures};
//!
//! // Build a custom context for testing
//! let context = IpContextBuilder::new()
//!     .ip("1.2.3.4")
//!     .infrastructure("DATACENTER")
//!     .vpn("NordVPN")
//!     .build();
//!
//! // Use pre-built fixtures
//! let residential = fixtures::residential_ip();
//! let vpn = fixtures::vpn_ip();
//! let tor = fixtures::tor_exit_node();
//! ```

use crate::{
    Ai, AutonomousSystem, Client, Concentration, IpContext, Location, Tunnel, TunnelEntry,
};

/// Builder for creating [`IpContext`] instances in tests.
///
/// Provides a fluent API for constructing test contexts with specific properties.
///
/// # Example
///
/// ```rust
/// use spur::test_utils::IpContextBuilder;
///
/// let context = IpContextBuilder::new()
///     .ip("89.39.106.191")
///     .infrastructure("DATACENTER")
///     .asn(49981, "WorldStream")
///     .add_risk("SPAM")
///     .build();
///
/// assert_eq!(context.ip.as_deref(), Some("89.39.106.191"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct IpContextBuilder {
    context: IpContext,
}

impl IpContextBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the IP address.
    pub fn ip(mut self, ip: &str) -> Self {
        self.context.ip = Some(ip.to_string());
        self
    }

    /// Set the infrastructure type (e.g., "DATACENTER", "RESIDENTIAL", "MOBILE").
    pub fn infrastructure(mut self, infra: &str) -> Self {
        self.context.infrastructure = Some(infra.to_string());
        self
    }

    /// Set the organization name.
    pub fn organization(mut self, org: &str) -> Self {
        self.context.organization = Some(org.to_string());
        self
    }

    /// Set autonomous system information.
    pub fn asn(mut self, number: u32, organization: &str) -> Self {
        self.context.autonomous_system = Some(AutonomousSystem {
            number: Some(number),
            organization: Some(organization.to_string()),
        });
        self
    }

    /// Set location information.
    pub fn location(mut self, country: &str, city: Option<&str>) -> Self {
        self.context.location = Some(Location {
            country: Some(country.to_string()),
            city: city.map(|s| s.to_string()),
            ..Default::default()
        });
        self
    }

    /// Set full location with coordinates.
    pub fn location_full(
        mut self,
        country: &str,
        state: Option<&str>,
        city: Option<&str>,
        lat: f64,
        lon: f64,
    ) -> Self {
        self.context.location = Some(Location {
            country: Some(country.to_string()),
            state: state.map(|s| s.to_string()),
            city: city.map(|s| s.to_string()),
            latitude: Some(lat),
            longitude: Some(lon),
        });
        self
    }

    /// Add a risk factor.
    pub fn add_risk(mut self, risk: &str) -> Self {
        let risks = self.context.risks.get_or_insert_with(Vec::new);
        risks.push(risk.to_string());
        self
    }

    /// Set multiple risk factors.
    pub fn risks(mut self, risks: &[&str]) -> Self {
        self.context.risks = Some(risks.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Add a service (e.g., "OPENVPN", "WIREGUARD", "IPSEC").
    pub fn add_service(mut self, service: &str) -> Self {
        let services = self.context.services.get_or_insert_with(Vec::new);
        services.push(service.to_string());
        self
    }

    /// Add a VPN tunnel with operator name.
    pub fn vpn(mut self, operator: &str) -> Self {
        let tunnels = self.context.tunnels.get_or_insert_with(Vec::new);
        tunnels.push(Tunnel {
            tunnel_type: Some("VPN".to_string()),
            operator: Some(operator.to_string()),
            anonymous: Some(true),
            entries: None,
        });
        self
    }

    /// Add a VPN tunnel with full details.
    pub fn vpn_with_entry(mut self, operator: &str, entry_ip: &str, entry_country: &str) -> Self {
        let tunnels = self.context.tunnels.get_or_insert_with(Vec::new);
        tunnels.push(Tunnel {
            tunnel_type: Some("VPN".to_string()),
            operator: Some(operator.to_string()),
            anonymous: Some(true),
            entries: Some(vec![TunnelEntry {
                ip: Some(entry_ip.to_string()),
                location: Some(Location {
                    country: Some(entry_country.to_string()),
                    ..Default::default()
                }),
                autonomous_system: None,
            }]),
        });
        self
    }

    /// Add a Tor exit node indicator.
    pub fn tor(mut self) -> Self {
        let tunnels = self.context.tunnels.get_or_insert_with(Vec::new);
        tunnels.push(Tunnel {
            tunnel_type: Some("TOR".to_string()),
            operator: Some("Tor Project".to_string()),
            anonymous: Some(true),
            entries: None,
        });
        self
    }

    /// Add a proxy indicator.
    pub fn proxy(mut self, operator: &str) -> Self {
        let tunnels = self.context.tunnels.get_or_insert_with(Vec::new);
        tunnels.push(Tunnel {
            tunnel_type: Some("PROXY".to_string()),
            operator: Some(operator.to_string()),
            anonymous: Some(false),
            entries: None,
        });
        self
    }

    /// Set AI scraper activity.
    pub fn ai_scraper(mut self, is_scraper: bool) -> Self {
        let ai = self.context.ai.get_or_insert_with(Ai::default);
        ai.scrapers = Some(is_scraper);
        self
    }

    /// Set AI bot activity with service names.
    pub fn ai_services(mut self, services: &[&str]) -> Self {
        let ai = self.context.ai.get_or_insert_with(Ai::default);
        ai.bots = Some(true);
        ai.services = Some(services.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set client information.
    pub fn client(mut self, count: u64, countries: u32) -> Self {
        let client = self.context.client.get_or_insert_with(Client::default);
        client.count = Some(count);
        client.countries = Some(countries);
        self
    }

    /// Set client behaviors.
    pub fn client_behaviors(mut self, behaviors: &[&str]) -> Self {
        let client = self.context.client.get_or_insert_with(Client::default);
        client.behaviors = Some(behaviors.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set client types.
    pub fn client_types(mut self, types: &[&str]) -> Self {
        let client = self.context.client.get_or_insert_with(Client::default);
        client.types = Some(types.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set geographic concentration.
    pub fn concentration(mut self, country: &str, city: &str, density: f64) -> Self {
        let client = self.context.client.get_or_insert_with(Client::default);
        client.concentration = Some(Concentration {
            country: Some(country.to_string()),
            city: Some(city.to_string()),
            density: Some(density),
            ..Default::default()
        });
        self
    }

    /// Build the final [`IpContext`].
    pub fn build(self) -> IpContext {
        self.context
    }
}

/// Pre-built test fixtures for common scenarios.
///
/// These fixtures represent typical IP contexts that you might encounter
/// in production and are useful for testing risk assessment logic.
pub mod fixtures {
    use super::*;

    /// A clean residential IP with no risk factors.
    ///
    /// Represents a typical home user connection.
    pub fn residential_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("203.0.113.1")
            .infrastructure("RESIDENTIAL")
            .asn(7922, "Comcast Cable")
            .location("US", Some("Philadelphia"))
            .client(1, 1)
            .client_types(&["DESKTOP"])
            .build()
    }

    /// A mobile network IP.
    ///
    /// Represents a cellular connection, may have multiple users.
    pub fn mobile_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("203.0.113.2")
            .infrastructure("MOBILE")
            .asn(310, "T-Mobile USA")
            .location("US", Some("Los Angeles"))
            .client(50, 1)
            .client_types(&["MOBILE"])
            .build()
    }

    /// A datacenter IP with no specific risk indicators.
    ///
    /// Could be a legitimate server or cloud instance.
    pub fn datacenter_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("198.51.100.1")
            .infrastructure("DATACENTER")
            .asn(16509, "Amazon Data Services")
            .location("US", Some("Ashburn"))
            .organization("AWS")
            .build()
    }

    /// A known VPN exit node.
    ///
    /// High risk for abuse, anonymous traffic.
    pub fn vpn_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("89.39.106.191")
            .infrastructure("DATACENTER")
            .asn(49981, "WorldStream")
            .location("NL", Some("Amsterdam"))
            .vpn("NordVPN")
            .add_risk("ANONYMOUS")
            .add_service("OPENVPN")
            .build()
    }

    /// A Tor exit node.
    ///
    /// Very high risk, fully anonymous traffic.
    pub fn tor_exit_node() -> IpContext {
        IpContextBuilder::new()
            .ip("185.220.101.1")
            .infrastructure("DATACENTER")
            .asn(60729, "Tor Exit")
            .location("DE", Some("Frankfurt"))
            .tor()
            .add_risk("ANONYMOUS")
            .add_risk("TOR_EXIT")
            .build()
    }

    /// A known proxy service IP.
    ///
    /// May be used by multiple clients.
    pub fn proxy_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("45.33.32.156")
            .infrastructure("DATACENTER")
            .asn(63949, "Linode")
            .proxy("Bright Data")
            .client(100, 15)
            .client_behaviors(&["PROXY_USER"])
            .add_risk("PROXY")
            .build()
    }

    /// An AI scraper IP (e.g., OpenAI, Anthropic crawler).
    pub fn ai_scraper_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("20.15.240.0")
            .infrastructure("DATACENTER")
            .asn(8075, "Microsoft Corporation")
            .organization("OpenAI")
            .ai_scraper(true)
            .ai_services(&["OPENAI", "CHATGPT"])
            .add_risk("AI_SCRAPER")
            .build()
    }

    /// A residential IP with proxy software installed.
    ///
    /// Part of a residential proxy network, very suspicious.
    pub fn residential_proxy_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("73.231.45.12")
            .infrastructure("RESIDENTIAL")
            .asn(7922, "Comcast Cable")
            .location("US", Some("Seattle"))
            .client(200, 45)
            .client_behaviors(&["FILE_SHARING", "TOR_PROXY_USER"])
            .concentration("RU", "Moscow", 0.85)
            .add_risk("RESIDENTIAL_PROXY")
            .build()
    }

    /// A clean corporate IP.
    ///
    /// Business network, single organization.
    pub fn corporate_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("17.253.144.10")
            .infrastructure("BUSINESS")
            .asn(714, "Apple Inc")
            .location("US", Some("Cupertino"))
            .organization("Apple Inc")
            .client(1, 1)
            .client_types(&["DESKTOP"])
            .build()
    }

    /// IP with multiple risk factors (worst case scenario).
    pub fn high_risk_ip() -> IpContext {
        IpContextBuilder::new()
            .ip("5.188.206.1")
            .infrastructure("DATACENTER")
            .asn(49505, "Selectel")
            .location("RU", Some("Moscow"))
            .vpn("Unknown VPN")
            .proxy("Luminati")
            .risks(&["ANONYMOUS", "SPAM", "SCAN", "ATTACK", "MALWARE"])
            .client(500, 80)
            .client_behaviors(&["SPAM", "SCAN", "ATTACK"])
            .build()
    }
}

/// Convert an [`IpContext`] to JSON for testing.
///
/// This is useful when you need to test JSON parsing or API mocking.
///
/// # Example
///
/// ```rust
/// use spur::test_utils::{to_json, fixtures};
///
/// let json = to_json(&fixtures::vpn_ip());
/// assert!(json.contains("NordVPN"));
/// ```
pub fn to_json(context: &IpContext) -> String {
    serde_json::to_string_pretty(context).expect("IpContext should serialize")
}

/// Parse JSON into an [`IpContext`] for testing.
///
/// Panics with a descriptive message if parsing fails.
pub fn from_json(json: &str) -> IpContext {
    serde_json::from_str(json).expect("Should parse as IpContext")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let context = IpContextBuilder::new()
            .ip("1.2.3.4")
            .infrastructure("DATACENTER")
            .build();

        assert_eq!(context.ip.as_deref(), Some("1.2.3.4"));
        assert_eq!(context.infrastructure.as_deref(), Some("DATACENTER"));
    }

    #[test]
    fn test_builder_with_vpn() {
        let context = IpContextBuilder::new().ip("1.2.3.4").vpn("NordVPN").build();

        let tunnels = context.tunnels.unwrap();
        assert_eq!(tunnels.len(), 1);
        assert_eq!(tunnels[0].tunnel_type.as_deref(), Some("VPN"));
        assert_eq!(tunnels[0].operator.as_deref(), Some("NordVPN"));
    }

    #[test]
    fn test_builder_multiple_tunnels() {
        let context = IpContextBuilder::new()
            .vpn("VPN1")
            .vpn("VPN2")
            .tor()
            .build();

        let tunnels = context.tunnels.unwrap();
        assert_eq!(tunnels.len(), 3);
    }

    #[test]
    fn test_fixtures_residential() {
        let ctx = fixtures::residential_ip();
        assert_eq!(ctx.infrastructure.as_deref(), Some("RESIDENTIAL"));
        assert!(ctx.tunnels.is_none());
    }

    #[test]
    fn test_fixtures_vpn() {
        let ctx = fixtures::vpn_ip();
        assert!(ctx.tunnels.is_some());
        let tunnels = ctx.tunnels.as_ref().unwrap();
        assert!(tunnels
            .iter()
            .any(|t| t.tunnel_type.as_deref() == Some("VPN")));
    }

    #[test]
    fn test_fixtures_tor() {
        let ctx = fixtures::tor_exit_node();
        let tunnels = ctx.tunnels.as_ref().unwrap();
        assert!(tunnels
            .iter()
            .any(|t| t.tunnel_type.as_deref() == Some("TOR")));
    }

    #[test]
    fn test_json_roundtrip() {
        let original = fixtures::high_risk_ip();
        let json = to_json(&original);
        let parsed = from_json(&json);

        assert_eq!(original.ip, parsed.ip);
        assert_eq!(original.infrastructure, parsed.infrastructure);
    }
}
