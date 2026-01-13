//! Integration tests for the spur crate.
//!
//! These tests verify end-to-end functionality using realistic API responses
//! and cross-module integration.

use spur::{
    Behavior, DeviceType, Infrastructure, IpContext, Risk, Service, TunnelType,
};

/// Test parsing a realistic VPN response from the Spur API.
#[test]
fn test_parse_realistic_vpn_response() {
    let json = include_str!("fixtures/vpn_response.json");
    let context: IpContext = serde_json::from_str(json).unwrap();

    assert_eq!(context.ip.as_deref(), Some("89.39.106.191"));
    assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));

    let asys = context.autonomous_system.as_ref().unwrap();
    assert_eq!(asys.number, Some(49981));
    assert_eq!(asys.organization.as_deref(), Some("WorldStream"));

    let tunnels = context.tunnels.as_ref().unwrap();
    assert_eq!(tunnels.len(), 1);
    assert_eq!(tunnels[0].tunnel_type, Some(TunnelType::Vpn));
    assert_eq!(tunnels[0].operator.as_deref(), Some("PROTON_VPN"));
    assert_eq!(tunnels[0].anonymous, Some(true));

    // Verify entries are parsed as simple IP strings
    let entries = tunnels[0].entries.as_ref().unwrap();
    assert!(!entries.is_empty());
    assert!(entries[0].ip.is_some());

    let risks = context.risks.as_ref().unwrap();
    assert!(risks.contains(&Risk::Tunnel));
}

/// Test parsing a residential IP response.
#[test]
fn test_parse_residential_response() {
    let json = include_str!("fixtures/residential_response.json");
    let context: IpContext = serde_json::from_str(json).unwrap();

    assert_eq!(context.ip.as_deref(), Some("203.0.113.45"));
    assert_eq!(context.infrastructure, Some(Infrastructure::Residential));
    assert!(context.tunnels.is_none());
    assert!(context.risks.is_none());

    let client = context.client.as_ref().unwrap();
    assert_eq!(client.count, Some(1));
    assert_eq!(client.types.as_ref().unwrap(), &vec![DeviceType::Desktop]);
}

/// Test parsing a Tor exit node response.
#[test]
fn test_parse_tor_response() {
    let json = include_str!("fixtures/tor_response.json");
    let context: IpContext = serde_json::from_str(json).unwrap();

    assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));

    let tunnels = context.tunnels.as_ref().unwrap();
    assert!(tunnels.iter().any(|t| t.tunnel_type == Some(TunnelType::Tor)));
}

/// Test parsing a datacenter IP with AI activity.
#[test]
fn test_parse_ai_scraper_response() {
    let json = include_str!("fixtures/ai_scraper_response.json");
    let context: IpContext = serde_json::from_str(json).unwrap();

    let ai = context.ai.as_ref().unwrap();
    assert_eq!(ai.scrapers, Some(true));
    assert!(ai.services.as_ref().unwrap().contains(&"OPENAI".to_string()));
}

/// Test parsing response with unknown enum values (forward compatibility).
#[test]
fn test_parse_unknown_enum_values() {
    let json = r#"{
        "ip": "1.2.3.4",
        "infrastructure": "SATELLITE",
        "risks": ["NEW_RISK_TYPE", "TUNNEL"],
        "services": ["QUANTUM_VPN"],
        "tunnels": [{"type": "WORMHOLE", "operator": "Future Corp"}]
    }"#;

    let context: IpContext = serde_json::from_str(json).unwrap();

    // Unknown infrastructure deserializes to Other
    assert_eq!(
        context.infrastructure,
        Some(Infrastructure::Other("SATELLITE".to_string()))
    );

    // Known and unknown risks both work
    let risks = context.risks.as_ref().unwrap();
    assert!(risks.contains(&Risk::Tunnel));
    assert!(risks.contains(&Risk::Other("NEW_RISK_TYPE".to_string())));

    // Unknown service
    let services = context.services.as_ref().unwrap();
    assert!(services.contains(&Service::Other("QUANTUM_VPN".to_string())));

    // Unknown tunnel type
    let tunnels = context.tunnels.as_ref().unwrap();
    assert_eq!(
        tunnels[0].tunnel_type,
        Some(TunnelType::Other("WORMHOLE".to_string()))
    );
}

/// Test that empty JSON parses to default context.
#[test]
fn test_parse_empty_json() {
    let context: IpContext = serde_json::from_str("{}").unwrap();

    assert!(context.ip.is_none());
    assert!(context.infrastructure.is_none());
    assert!(context.risks.is_none());
    assert!(context.tunnels.is_none());
}

/// Test roundtrip serialization preserves data.
#[test]
fn test_roundtrip_serialization() {
    let json = include_str!("fixtures/vpn_response.json");
    let original: IpContext = serde_json::from_str(json).unwrap();

    let serialized = serde_json::to_string(&original).unwrap();
    let reparsed: IpContext = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original.ip, reparsed.ip);
    assert_eq!(original.infrastructure, reparsed.infrastructure);
    assert_eq!(original.risks, reparsed.risks);
}

/// Test parsing response with client behaviors.
#[test]
fn test_parse_client_behaviors() {
    let json = r#"{
        "ip": "1.2.3.4",
        "client": {
            "behaviors": ["FILE_SHARING", "TOR_PROXY_USER", "NEW_BEHAVIOR"],
            "types": ["MOBILE", "DESKTOP", "TABLET"],
            "count": 150,
            "countries": 12
        }
    }"#;

    let context: IpContext = serde_json::from_str(json).unwrap();
    let client = context.client.as_ref().unwrap();

    let behaviors = client.behaviors.as_ref().unwrap();
    assert!(behaviors.contains(&Behavior::FileSharing));
    assert!(behaviors.contains(&Behavior::TorProxyUser));
    assert!(behaviors.contains(&Behavior::Other("NEW_BEHAVIOR".to_string())));

    let types = client.types.as_ref().unwrap();
    assert!(types.contains(&DeviceType::Mobile));
    assert!(types.contains(&DeviceType::Desktop));
    assert!(types.contains(&DeviceType::Other("TABLET".to_string())));

    assert_eq!(client.count, Some(150));
    assert_eq!(client.countries, Some(12));
}

/// Test using test utilities with integration tests.
#[test]
fn test_builder_integration() {
    use spur::test_utils::{fixtures, IpContextBuilder};

    // Build a custom context
    let context = IpContextBuilder::new()
        .ip("10.0.0.1")
        .infrastructure(Infrastructure::Business)
        .asn(12345, "Test Corp")
        .add_risk(Risk::Spam)
        .add_service(Service::Ssh)
        .build();

    // Serialize and reparse
    let json = serde_json::to_string(&context).unwrap();
    let parsed: IpContext = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.infrastructure, Some(Infrastructure::Business));
    assert!(parsed.risks.as_ref().unwrap().contains(&Risk::Spam));
    assert!(parsed.services.as_ref().unwrap().contains(&Service::Ssh));

    // Use fixtures
    let vpn = fixtures::vpn_ip();
    assert!(vpn.tunnels.is_some());
}

/// Test that None fields are omitted during serialization.
#[test]
fn test_none_fields_omitted() {
    let context = IpContext {
        ip: Some("1.2.3.4".to_string()),
        infrastructure: Some(Infrastructure::Datacenter),
        ..Default::default()
    };

    let json = serde_json::to_string(&context).unwrap();

    // Should NOT contain fields that are None
    assert!(!json.contains("risks"));
    assert!(!json.contains("tunnels"));
    assert!(!json.contains("client"));
    assert!(!json.contains("ai"));

    // Should contain fields that are Some
    assert!(json.contains("\"ip\""));
    assert!(json.contains("\"infrastructure\""));
}
