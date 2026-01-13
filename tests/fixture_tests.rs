//! Fixture-based tests for verifying parsing of real Spur API responses.
//!
//! This module tests parsing of JSON fixtures saved from real API responses.
//! This ensures the library correctly handles actual production data.
//!
//! # Adding New Fixtures
//!
//! To add a new fixture from the Spur API:
//!
//! 1. Save the JSON response to `tests/fixtures/<descriptive_name>.json`
//! 2. The fixture will automatically be tested for:
//!    - Valid JSON parsing to `IpContext`
//!    - Round-trip serialization (deserialize -> serialize -> deserialize)
//!    - Non-empty IP field (if present)
//!
//! # Fixture Naming Convention
//!
//! Use descriptive names that indicate the type of IP:
//! - `vpn_nordvpn.json` - VPN exit node
//! - `residential_comcast.json` - Residential ISP
//! - `tor_exit_de.json` - Tor exit node in Germany
//! - `datacenter_aws.json` - AWS datacenter IP
//! - `mobile_verizon.json` - Mobile carrier IP
//!
//! # Example: Saving a fixture from curl
//!
//! ```bash
//! curl -s "https://api.spur.us/v2/context/<IP>" \
//!   -H "Token: YOUR_API_TOKEN" \
//!   | jq . > tests/fixtures/my_new_fixture.json
//! ```

use std::fs;
use std::path::PathBuf;

use spur::IpContext;

/// Get all JSON fixture files from the fixtures directory.
fn get_fixture_files() -> Vec<PathBuf> {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");

    fs::read_dir(&fixtures_dir)
        .expect("Failed to read fixtures directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
        .collect()
}

/// Test that all fixtures can be parsed as valid IpContext.
#[test]
fn test_all_fixtures_parse() {
    let fixtures = get_fixture_files();
    assert!(!fixtures.is_empty(), "No fixture files found in tests/fixtures/");

    for fixture_path in &fixtures {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", filename, e));

        let result: Result<IpContext, _> = serde_json::from_str(&json);
        assert!(
            result.is_ok(),
            "Failed to parse fixture {}: {}",
            filename,
            result.unwrap_err()
        );

        let context = result.unwrap();
        println!("✓ Parsed {}: ip={:?}", filename, context.ip);
    }

    println!("\n✓ All {} fixtures parsed successfully", fixtures.len());
}

/// Test that all fixtures can be round-tripped through serialization.
#[test]
fn test_all_fixtures_roundtrip() {
    let fixtures = get_fixture_files();

    for fixture_path in &fixtures {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        // Parse the original
        let original: IpContext = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", filename, e));

        // Serialize back to JSON
        let serialized = serde_json::to_string(&original)
            .unwrap_or_else(|e| panic!("Failed to serialize {}: {}", filename, e));

        // Parse again
        let reparsed: IpContext = serde_json::from_str(&serialized)
            .unwrap_or_else(|e| panic!("Failed to reparse {}: {}", filename, e));

        // Verify equality
        assert_eq!(
            original, reparsed,
            "Round-trip mismatch for {}",
            filename
        );

        println!("✓ Round-trip successful for {}", filename);
    }
}

/// Test that fixtures with IP addresses have valid-looking IPs.
#[test]
fn test_fixtures_have_valid_ips() {
    let fixtures = get_fixture_files();

    for fixture_path in &fixtures {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        if let Some(ip) = &context.ip {
            // Basic validation: should contain dots (IPv4) or colons (IPv6)
            assert!(
                ip.contains('.') || ip.contains(':'),
                "Invalid IP format in {}: {}",
                filename,
                ip
            );

            // Should not be empty
            assert!(!ip.is_empty(), "Empty IP in {}", filename);

            println!("✓ Valid IP in {}: {}", filename, ip);
        }
    }
}

/// Test that VPN fixtures have tunnel information.
/// Note: Only matches files starting with "vpn_", not "rvpn_" (residential proxy VPNs).
#[test]
fn test_vpn_fixtures_have_tunnels() {
    let fixtures = get_fixture_files();

    for fixture_path in fixtures.iter().filter(|p| {
        let name = p.file_name().unwrap().to_string_lossy();
        // Match vpn_*.json but not rvpn_*.json (residential proxy)
        name.starts_with("vpn_") || name.ends_with("_vpn.json")
    }) {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        assert!(
            context.tunnels.is_some() && !context.tunnels.as_ref().unwrap().is_empty(),
            "VPN fixture {} should have tunnel information",
            filename
        );

        println!(
            "✓ VPN fixture {} has {} tunnel(s)",
            filename,
            context.tunnels.as_ref().unwrap().len()
        );
    }
}

/// Test that Tor fixtures have Tor tunnel type.
#[test]
fn test_tor_fixtures_have_tor_tunnel() {
    use spur::TunnelType;

    let fixtures = get_fixture_files();

    for fixture_path in fixtures
        .iter()
        .filter(|p| p.file_name().unwrap().to_string_lossy().contains("tor"))
    {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        let has_tor = context
            .tunnels
            .as_ref()
            .map(|tunnels| {
                tunnels
                    .iter()
                    .any(|t| t.tunnel_type == Some(TunnelType::Tor))
            })
            .unwrap_or(false);

        assert!(
            has_tor,
            "Tor fixture {} should have TOR tunnel type",
            filename
        );

        println!("✓ Tor fixture {} has Tor tunnel", filename);
    }
}

/// Test that residential fixtures have residential infrastructure.
#[test]
fn test_residential_fixtures_have_residential_infra() {
    use spur::Infrastructure;

    let fixtures = get_fixture_files();

    for fixture_path in fixtures.iter().filter(|p| {
        p.file_name()
            .unwrap()
            .to_string_lossy()
            .contains("residential")
    }) {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        assert_eq!(
            context.infrastructure,
            Some(Infrastructure::Residential),
            "Residential fixture {} should have RESIDENTIAL infrastructure",
            filename
        );

        println!("✓ Residential fixture {} has correct infrastructure", filename);
    }
}

/// Test that AI scraper fixtures have AI information.
/// Note: Matches files starting with "ai_" or containing "scraper" or "bot".
#[test]
fn test_ai_fixtures_have_ai_info() {
    let fixtures = get_fixture_files();

    for fixture_path in fixtures.iter().filter(|p| {
        let name = p.file_name().unwrap().to_string_lossy();
        // Match ai_*.json or *scraper*.json or *bot*.json
        name.starts_with("ai_") || name.contains("scraper") || name.contains("bot")
    }) {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        assert!(
            context.ai.is_some(),
            "AI fixture {} should have AI information",
            filename
        );

        let ai = context.ai.as_ref().unwrap();
        let has_ai_activity =
            ai.scrapers == Some(true) || ai.bots == Some(true) || ai.services.is_some();

        assert!(
            has_ai_activity,
            "AI fixture {} should have some AI activity",
            filename
        );

        println!("✓ AI fixture {} has AI information", filename);
    }
}

/// Print a summary of all fixture files and their key properties.
#[test]
fn test_fixture_summary() {
    let fixtures = get_fixture_files();

    println!("\n=== Fixture Summary ===\n");

    for fixture_path in &fixtures {
        let filename = fixture_path.file_name().unwrap().to_string_lossy();
        let json = fs::read_to_string(fixture_path).unwrap();

        let context: IpContext = serde_json::from_str(&json).unwrap();

        println!("📄 {}", filename);
        if let Some(ip) = &context.ip {
            println!("   IP: {}", ip);
        }
        if let Some(infra) = &context.infrastructure {
            println!("   Infrastructure: {:?}", infra);
        }
        if let Some(org) = &context.organization {
            println!("   Organization: {}", org);
        }
        if let Some(risks) = &context.risks {
            println!("   Risks: {:?}", risks);
        }
        if let Some(tunnels) = &context.tunnels {
            let types: Vec<_> = tunnels.iter().filter_map(|t| t.tunnel_type.as_ref()).collect();
            println!("   Tunnels: {:?}", types);
        }
        if let Some(ai) = &context.ai {
            println!("   AI: scrapers={:?}, bots={:?}", ai.scrapers, ai.bots);
        }
        println!();
    }

    println!("Total fixtures: {}", fixtures.len());
}

#[cfg(test)]
mod individual_fixture_tests {
    //! Individual tests for specific fixtures.
    //! These provide more detailed assertions for known fixture files.

    use super::*;
    use spur::{Infrastructure, Risk, TunnelType};

    #[test]
    fn test_vpn_response_fixture() {
        let json = include_str!("fixtures/vpn_response.json");
        let context: IpContext = serde_json::from_str(json).unwrap();

        // Verify key VPN properties
        assert_eq!(context.ip.as_deref(), Some("89.39.106.191"));
        assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));

        // Should have VPN tunnel
        let tunnels = context.tunnels.as_ref().expect("VPN should have tunnels");
        assert!(tunnels.iter().any(|t| t.tunnel_type == Some(TunnelType::Vpn)));

        // Should have tunnel risk
        let risks = context.risks.as_ref().expect("VPN should have risks");
        assert!(risks.contains(&Risk::Tunnel));
    }

    #[test]
    fn test_residential_response_fixture() {
        let json = include_str!("fixtures/residential_response.json");
        let context: IpContext = serde_json::from_str(json).unwrap();

        assert_eq!(context.infrastructure, Some(Infrastructure::Residential));

        // Residential IPs typically don't have tunnels
        assert!(context.tunnels.is_none());

        // Should have client info
        assert!(context.client.is_some());
    }

    #[test]
    fn test_tor_response_fixture() {
        let json = include_str!("fixtures/tor_response.json");
        let context: IpContext = serde_json::from_str(json).unwrap();

        // Should have Tor tunnel
        let tunnels = context.tunnels.as_ref().expect("Tor should have tunnels");
        assert!(tunnels.iter().any(|t| t.tunnel_type == Some(TunnelType::Tor)));

        // Should have high-risk indicators
        let risks = context.risks.as_ref().expect("Tor should have risks");
        assert!(risks.contains(&Risk::Tunnel));
    }

    #[test]
    fn test_ai_scraper_response_fixture() {
        let json = include_str!("fixtures/ai_scraper_response.json");
        let context: IpContext = serde_json::from_str(json).unwrap();

        let ai = context.ai.as_ref().expect("AI fixture should have AI info");
        assert_eq!(ai.scrapers, Some(true));

        // Should have AI services
        let services = ai.services.as_ref().expect("Should have AI services");
        assert!(!services.is_empty());
    }
}
