//! Property-based fuzz tests using proptest.
//!
//! These tests verify invariants across randomly generated inputs.

use proptest::prelude::*;
use spur::proptest_strategies::*;
use spur::{IpContext, Infrastructure, Risk, Service, TunnelType};

proptest! {
    /// Verify that all generated IpContext values can roundtrip through JSON.
    #[test]
    fn fuzz_ip_context_json_roundtrip(context in arb_ip_context()) {
        let json = serde_json::to_string(&context).unwrap();
        let parsed: IpContext = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(context, parsed);
    }

    /// Verify that minimal contexts roundtrip correctly.
    #[test]
    fn fuzz_minimal_context_roundtrip(context in arb_minimal_ip_context()) {
        let json = serde_json::to_string(&context).unwrap();
        let parsed: IpContext = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(context, parsed);
    }

    /// Verify that VPN contexts roundtrip correctly.
    #[test]
    fn fuzz_vpn_context_roundtrip(context in arb_vpn_context()) {
        let json = serde_json::to_string(&context).unwrap();
        let parsed: IpContext = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(context, parsed);
    }

    /// Verify infrastructure enum display matches as_str.
    #[test]
    fn fuzz_infrastructure_display_consistency(infra in arb_infrastructure()) {
        let display = format!("{}", infra);
        let as_str = infra.as_str();
        prop_assert_eq!(display, as_str);
    }

    /// Verify risk enum display matches as_str.
    #[test]
    fn fuzz_risk_display_consistency(risk in arb_risk()) {
        let display = format!("{}", risk);
        let as_str = risk.as_str();
        prop_assert_eq!(display, as_str);
    }

    /// Verify service enum display matches as_str.
    #[test]
    fn fuzz_service_display_consistency(service in arb_service()) {
        let display = format!("{}", service);
        let as_str = service.as_str();
        prop_assert_eq!(display, as_str);
    }

    /// Verify tunnel type enum display matches as_str.
    #[test]
    fn fuzz_tunnel_type_display_consistency(tt in arb_tunnel_type()) {
        let display = format!("{}", tt);
        let as_str = tt.as_str();
        prop_assert_eq!(display, as_str);
    }

    /// Verify that serialized JSON is valid JSON.
    #[test]
    fn fuzz_json_validity(context in arb_ip_context()) {
        let json = serde_json::to_string(&context).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        prop_assert!(value.is_object());
    }

    /// Verify that serialized JSON contains ip field when present.
    #[test]
    fn fuzz_json_contains_ip(context in arb_ip_context()) {
        let json = serde_json::to_string(&context).unwrap();
        if context.ip.is_some() {
            prop_assert!(json.contains("\"ip\""));
        }
    }

    /// Verify that is_other returns true only for Other variants.
    #[test]
    fn fuzz_infrastructure_is_other(infra in arb_infrastructure()) {
        match &infra {
            Infrastructure::Other(_) => prop_assert!(infra.is_other()),
            _ => prop_assert!(!infra.is_other()),
        }
    }

    /// Verify that is_other returns true only for Other variants.
    #[test]
    fn fuzz_risk_is_other(risk in arb_risk()) {
        match &risk {
            Risk::Other(_) => prop_assert!(risk.is_other()),
            _ => prop_assert!(!risk.is_other()),
        }
    }

    /// Verify that is_other returns true only for Other variants.
    #[test]
    fn fuzz_service_is_other(service in arb_service()) {
        match &service {
            Service::Other(_) => prop_assert!(service.is_other()),
            _ => prop_assert!(!service.is_other()),
        }
    }

    /// Verify that is_other returns true only for Other variants.
    #[test]
    fn fuzz_tunnel_type_is_other(tt in arb_tunnel_type()) {
        match &tt {
            TunnelType::Other(_) => prop_assert!(tt.is_other()),
            _ => prop_assert!(!tt.is_other()),
        }
    }
}

/// Test that arbitrary JSON strings don't panic during parsing (resilience test).
#[test]
fn test_invalid_json_handling() {
    let invalid_inputs = vec![
        "",
        "null",
        "[]",
        "\"string\"",
        "123",
        "{invalid}",
        "{\"ip\": }",
        "{\"infrastructure\": 123}",
    ];

    for input in invalid_inputs {
        // Should not panic, just return an error
        let _ = serde_json::from_str::<IpContext>(input);
    }
}

/// Test edge cases with empty collections.
#[test]
fn test_empty_collections() {
    let json = r#"{
        "ip": "1.2.3.4",
        "risks": [],
        "services": [],
        "tunnels": [],
        "client": {
            "behaviors": [],
            "types": [],
            "proxies": []
        }
    }"#;

    let context: IpContext = serde_json::from_str(json).unwrap();

    assert_eq!(context.risks.as_ref().unwrap().len(), 0);
    assert_eq!(context.services.as_ref().unwrap().len(), 0);
    assert_eq!(context.tunnels.as_ref().unwrap().len(), 0);

    let client = context.client.as_ref().unwrap();
    assert_eq!(client.behaviors.as_ref().unwrap().len(), 0);
    assert_eq!(client.types.as_ref().unwrap().len(), 0);
}

/// Test that very large values don't cause issues.
#[test]
fn test_large_values() {
    let json = r#"{
        "ip": "255.255.255.255",
        "as": {
            "number": 4294967295,
            "organization": "Max AS Number"
        },
        "client": {
            "count": 18446744073709551615,
            "countries": 200,
            "spread": 18446744073709551615
        },
        "location": {
            "latitude": 90.0,
            "longitude": 180.0
        }
    }"#;

    let context: IpContext = serde_json::from_str(json).unwrap();
    let asys = context.autonomous_system.as_ref().unwrap();
    assert_eq!(asys.number, Some(u32::MAX));

    let client = context.client.as_ref().unwrap();
    assert_eq!(client.count, Some(u64::MAX));
}
