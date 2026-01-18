//! Proptest strategies for generating arbitrary Spur API types.
//!
//! This module provides [`Arbitrary`] implementations and custom strategies
//! for property-based testing with proptest.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use proptest::prelude::*;
//! use spur::proptest_strategies::*;
//!
//! proptest! {
//!     #[test]
//!     fn roundtrip_serialization(context in arb_ip_context()) {
//!         let json = serde_json::to_string(&context).unwrap();
//!         let parsed: IpContext = serde_json::from_str(&json).unwrap();
//!         assert_eq!(context, parsed);
//!     }
//! }
//! ```

use proptest::prelude::*;

use crate::context::{
    Ai, AutonomousSystem, Behavior, Client, Concentration, DeviceType, Infrastructure, IpContext,
    Location, Risk, Service, Tunnel, TunnelEntry, TunnelType,
};
use crate::monocle::Assessment;

// =============================================================================
// Context API Strategies
// =============================================================================

/// Strategy for generating arbitrary Infrastructure values.
pub fn arb_infrastructure() -> impl Strategy<Value = Infrastructure> {
    prop_oneof![
        Just(Infrastructure::Datacenter),
        Just(Infrastructure::Residential),
        Just(Infrastructure::Mobile),
        Just(Infrastructure::Business),
        "[A-Z_]{3,20}".prop_map(Infrastructure::Other),
    ]
}

/// Strategy for generating arbitrary Risk values.
pub fn arb_risk() -> impl Strategy<Value = Risk> {
    prop_oneof![
        Just(Risk::Tunnel),
        Just(Risk::Spam),
        Just(Risk::CallbackProxy),
        Just(Risk::GeoMismatch),
        "[A-Z_]{3,20}".prop_map(Risk::Other),
    ]
}

/// Strategy for generating arbitrary Service values.
pub fn arb_service() -> impl Strategy<Value = Service> {
    prop_oneof![
        Just(Service::OpenVpn),
        Just(Service::Ipsec),
        Just(Service::Wireguard),
        Just(Service::Ssh),
        Just(Service::Pptp),
        "[A-Z_]{3,20}".prop_map(Service::Other),
    ]
}

/// Strategy for generating arbitrary TunnelType values.
pub fn arb_tunnel_type() -> impl Strategy<Value = TunnelType> {
    prop_oneof![
        Just(TunnelType::Vpn),
        Just(TunnelType::Proxy),
        Just(TunnelType::Tor),
        "[A-Z_]{3,20}".prop_map(TunnelType::Other),
    ]
}

/// Strategy for generating arbitrary Behavior values.
pub fn arb_behavior() -> impl Strategy<Value = Behavior> {
    prop_oneof![
        Just(Behavior::FileSharing),
        Just(Behavior::TorProxyUser),
        "[A-Z_]{3,20}".prop_map(Behavior::Other),
    ]
}

/// Strategy for generating arbitrary DeviceType values.
pub fn arb_device_type() -> impl Strategy<Value = DeviceType> {
    prop_oneof![
        Just(DeviceType::Mobile),
        Just(DeviceType::Desktop),
        "[A-Z_]{3,20}".prop_map(DeviceType::Other),
    ]
}

/// Strategy for generating arbitrary Location values.
///
/// Uses integer-based coordinates to avoid floating-point precision issues
/// in JSON roundtrip testing.
pub fn arb_location() -> impl Strategy<Value = Location> {
    (
        proptest::option::of("[A-Z]{2}"),
        proptest::option::of("[A-Za-z ]{2,30}"),
        proptest::option::of("[A-Za-z ]{2,30}"),
        proptest::option::of(-90i32..90i32),
        proptest::option::of(-180i32..180i32),
    )
        .prop_map(|(country, state, city, lat, lon)| Location {
            country,
            state,
            city,
            latitude: lat.map(|v| v as f64),
            longitude: lon.map(|v| v as f64),
        })
}

/// Strategy for generating arbitrary AutonomousSystem values.
pub fn arb_autonomous_system() -> impl Strategy<Value = AutonomousSystem> {
    (
        proptest::option::of(1u32..400000),
        proptest::option::of("[A-Za-z0-9 ]{2,50}"),
    )
        .prop_map(|(number, organization)| AutonomousSystem {
            number,
            organization,
        })
}

/// Strategy for generating arbitrary Concentration values.
///
/// Uses integer-based density (divided by 100) to avoid floating-point
/// precision issues in JSON roundtrip testing.
pub fn arb_concentration() -> impl Strategy<Value = Concentration> {
    (
        proptest::option::of("[A-Z]{2}"),
        proptest::option::of("[A-Za-z ]{2,30}"),
        proptest::option::of("[A-Za-z ]{2,30}"),
        proptest::option::of(0u32..100u32),
        proptest::option::of("[a-z0-9]{3,12}"),
        proptest::option::of(0u64..10000),
    )
        .prop_map(
            |(country, state, city, density, geohash, skew)| Concentration {
                country,
                state,
                city,
                density: density.map(|v| v as f64 / 100.0),
                geohash,
                skew,
            },
        )
}

/// Strategy for generating arbitrary TunnelEntry values.
pub fn arb_tunnel_entry() -> impl Strategy<Value = TunnelEntry> {
    (
        proptest::option::of("[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}"),
        proptest::option::of(arb_location()),
        proptest::option::of(arb_autonomous_system()),
    )
        .prop_map(|(ip, location, autonomous_system)| TunnelEntry {
            ip,
            location,
            autonomous_system,
        })
}

/// Strategy for generating arbitrary Tunnel values.
pub fn arb_tunnel() -> impl Strategy<Value = Tunnel> {
    (
        proptest::option::of(arb_tunnel_type()),
        proptest::option::of("[A-Za-z0-9 ]{2,30}"),
        proptest::option::of(proptest::bool::ANY),
        proptest::option::of(proptest::collection::vec(arb_tunnel_entry(), 0..3)),
    )
        .prop_map(|(tunnel_type, operator, anonymous, entries)| Tunnel {
            tunnel_type,
            operator,
            anonymous,
            entries,
        })
}

/// Strategy for generating arbitrary Ai values.
pub fn arb_ai() -> impl Strategy<Value = Ai> {
    (
        proptest::option::of(proptest::bool::ANY),
        proptest::option::of(proptest::bool::ANY),
        proptest::option::of(proptest::collection::vec("[A-Z]{2,20}", 0..5)),
    )
        .prop_map(|(scrapers, bots, services)| Ai {
            scrapers,
            bots,
            services,
        })
}

/// Strategy for generating arbitrary Client values.
pub fn arb_client() -> impl Strategy<Value = Client> {
    (
        proptest::option::of(proptest::collection::vec(arb_behavior(), 0..5)),
        proptest::option::of(arb_concentration()),
        proptest::option::of(0u64..10000),
        proptest::option::of(0u32..200),
        proptest::option::of(proptest::collection::vec("[A-Z_]{5,30}", 0..5)),
        proptest::option::of(0u64..10000000),
        proptest::option::of(proptest::collection::vec(arb_device_type(), 0..3)),
    )
        .prop_map(
            |(behaviors, concentration, count, countries, proxies, spread, types)| Client {
                behaviors,
                concentration,
                count,
                countries,
                proxies,
                spread,
                types,
            },
        )
}

/// Strategy for generating arbitrary IpContext values.
///
/// This generates fully random contexts, including all optional fields.
pub fn arb_ip_context() -> impl Strategy<Value = IpContext> {
    (
        proptest::option::of(arb_ai()),
        proptest::option::of(arb_autonomous_system()),
        proptest::option::of(arb_client()),
        proptest::option::of(arb_infrastructure()),
        proptest::option::of("[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}"),
        proptest::option::of(arb_location()),
        proptest::option::of("[A-Za-z0-9 ]{2,50}"),
        proptest::option::of(proptest::collection::vec(arb_risk(), 0..5)),
        proptest::option::of(proptest::collection::vec(arb_service(), 0..5)),
        proptest::option::of(proptest::collection::vec(arb_tunnel(), 0..3)),
    )
        .prop_map(
            |(
                ai,
                autonomous_system,
                client,
                infrastructure,
                ip,
                location,
                organization,
                risks,
                services,
                tunnels,
            )| {
                IpContext {
                    ai,
                    autonomous_system,
                    client,
                    infrastructure,
                    ip,
                    location,
                    organization,
                    risks,
                    services,
                    tunnels,
                }
            },
        )
}

/// Strategy for generating minimal IpContext (just IP).
pub fn arb_minimal_ip_context() -> impl Strategy<Value = IpContext> {
    "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}".prop_map(|ip| IpContext {
        ip: Some(ip),
        ..Default::default()
    })
}

/// Strategy for generating realistic VPN contexts.
pub fn arb_vpn_context() -> impl Strategy<Value = IpContext> {
    (
        "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}",
        "[A-Za-z ]{3,20}",
        proptest::collection::vec(arb_service(), 1..3),
    )
        .prop_map(|(ip, operator, services)| IpContext {
            ip: Some(ip),
            infrastructure: Some(Infrastructure::Datacenter),
            tunnels: Some(vec![Tunnel {
                tunnel_type: Some(TunnelType::Vpn),
                operator: Some(operator),
                anonymous: Some(true),
                entries: None,
            }]),
            risks: Some(vec![Risk::Tunnel]),
            services: Some(services),
            ..Default::default()
        })
}

// =============================================================================
// Monocle API Strategies
// =============================================================================

/// Strategy for generating arbitrary Assessment values.
pub fn arb_assessment() -> impl Strategy<Value = Assessment> {
    (
        proptest::bool::ANY,
        proptest::bool::ANY,
        proptest::bool::ANY,
        "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}",
        "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
        proptest::bool::ANY,
        "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}",
        "[a-z0-9-]{3,30}",
    )
        .prop_map(|(vpn, proxied, anon, ip, ts, complete, id, sid)| Assessment {
            vpn,
            proxied,
            anon,
            ip,
            ts,
            complete,
            id,
            sid,
        })
}

/// Strategy for generating clean (non-anonymous) assessments.
pub fn arb_clean_assessment() -> impl Strategy<Value = Assessment> {
    (
        "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}",
        "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
        "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}",
        "[a-z0-9-]{3,30}",
    )
        .prop_map(|(ip, ts, id, sid)| Assessment {
            vpn: false,
            proxied: false,
            anon: false,
            ip,
            ts,
            complete: true,
            id,
            sid,
        })
}

/// Strategy for generating VPN-detected assessments.
pub fn arb_vpn_assessment() -> impl Strategy<Value = Assessment> {
    (
        "[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}",
        "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
        "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}",
        "[a-z0-9-]{3,30}",
    )
        .prop_map(|(ip, ts, id, sid)| Assessment {
            vpn: true,
            proxied: false,
            anon: true,
            ip,
            ts,
            complete: true,
            id,
            sid,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn infrastructure_roundtrip(infra in arb_infrastructure()) {
            let json = serde_json::to_string(&infra).unwrap();
            let parsed: Infrastructure = serde_json::from_str(&json).unwrap();
            assert_eq!(infra, parsed);
        }

        #[test]
        fn risk_roundtrip(risk in arb_risk()) {
            let json = serde_json::to_string(&risk).unwrap();
            let parsed: Risk = serde_json::from_str(&json).unwrap();
            assert_eq!(risk, parsed);
        }

        #[test]
        fn service_roundtrip(service in arb_service()) {
            let json = serde_json::to_string(&service).unwrap();
            let parsed: Service = serde_json::from_str(&json).unwrap();
            assert_eq!(service, parsed);
        }

        #[test]
        fn tunnel_type_roundtrip(tt in arb_tunnel_type()) {
            let json = serde_json::to_string(&tt).unwrap();
            let parsed: TunnelType = serde_json::from_str(&json).unwrap();
            assert_eq!(tt, parsed);
        }

        #[test]
        fn behavior_roundtrip(behavior in arb_behavior()) {
            let json = serde_json::to_string(&behavior).unwrap();
            let parsed: Behavior = serde_json::from_str(&json).unwrap();
            assert_eq!(behavior, parsed);
        }

        #[test]
        fn device_type_roundtrip(device in arb_device_type()) {
            let json = serde_json::to_string(&device).unwrap();
            let parsed: DeviceType = serde_json::from_str(&json).unwrap();
            assert_eq!(device, parsed);
        }

        #[test]
        fn ip_context_roundtrip(context in arb_ip_context()) {
            let json = serde_json::to_string(&context).unwrap();
            let parsed: IpContext = serde_json::from_str(&json).unwrap();
            assert_eq!(context, parsed);
        }

        #[test]
        fn minimal_context_roundtrip(context in arb_minimal_ip_context()) {
            let json = serde_json::to_string(&context).unwrap();
            let parsed: IpContext = serde_json::from_str(&json).unwrap();
            assert_eq!(context, parsed);
        }

        #[test]
        fn vpn_context_roundtrip(context in arb_vpn_context()) {
            let json = serde_json::to_string(&context).unwrap();
            let parsed: IpContext = serde_json::from_str(&json).unwrap();
            assert_eq!(context, parsed);
        }

        #[test]
        fn infrastructure_display_matches_serialization(infra in arb_infrastructure()) {
            let display = format!("{}", infra);
            let serialized: String = serde_json::from_str(&serde_json::to_string(&infra).unwrap()).unwrap();
            assert_eq!(display, serialized);
        }

        // Monocle API tests
        #[test]
        fn assessment_roundtrip(assessment in arb_assessment()) {
            let json = serde_json::to_string(&assessment).unwrap();
            let parsed: Assessment = serde_json::from_str(&json).unwrap();
            assert_eq!(assessment, parsed);
        }

        #[test]
        fn clean_assessment_roundtrip(assessment in arb_clean_assessment()) {
            let json = serde_json::to_string(&assessment).unwrap();
            let parsed: Assessment = serde_json::from_str(&json).unwrap();
            assert_eq!(assessment, parsed);
            assert!(!parsed.is_anonymized());
        }

        #[test]
        fn vpn_assessment_roundtrip(assessment in arb_vpn_assessment()) {
            let json = serde_json::to_string(&assessment).unwrap();
            let parsed: Assessment = serde_json::from_str(&json).unwrap();
            assert_eq!(assessment, parsed);
            assert!(parsed.vpn);
            assert!(parsed.is_anonymized());
        }
    }
}
