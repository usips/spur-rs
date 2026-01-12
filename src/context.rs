//! IP Context Object types for the Spur Context API.

use serde::{Deserialize, Serialize};

use crate::enums::{Behavior, DeviceType, Infrastructure, Risk, Service, TunnelType};

/// The IP Context Object summarizes all available information for an IP address.
///
/// All fields may be omitted if their value is null.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IpContext {
    /// A top-level field describing AI activity observed from this IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai: Option<Ai>,

    /// BGP autonomous system information.
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub autonomous_system: Option<AutonomousSystem>,

    /// Descriptive data about the connecting client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Client>,

    /// Infrastructure type classification (datacenter, residential, mobile, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<Infrastructure>,

    /// IPv4 or IPv6 address associated with the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Spur IP Geo location information of the IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// The organization currently assigned to use the specific IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// List of identified risk factors or behaviors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risks: Option<Vec<Risk>>,

    /// List of services or protocols in use (OpenVPN, IPSec, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    /// Information about tunneling methods (VPN, TOR, etc.) used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnels: Option<Vec<Tunnel>>,
}

/// AI activity observed from an IP address.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ai {
    /// Whether AI scraper activity has been observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrapers: Option<bool>,

    /// Whether AI bot activity has been observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bots: Option<bool>,

    /// List of AI services observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

/// BGP autonomous system information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutonomousSystem {
    /// The autonomous system number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// The organization name for this AS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

/// Descriptive data about the connecting client.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Client {
    /// Observed client behaviors (file sharing, tor usage, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,

    /// Geographic concentration of users behind this IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concentration: Option<Concentration>,

    /// Number of distinct clients observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    /// Number of distinct countries observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<u32>,

    /// Proxy services observed (service-specific identifiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxies: Option<Vec<String>>,

    /// Geographic spread metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<u64>,

    /// Client device types observed (mobile, desktop, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<DeviceType>>,
}

/// Geographic concentration of users behind an IP.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Concentration {
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Country code (ISO 3166-1 alpha-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Density metric (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,

    /// Geohash of the concentration area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geohash: Option<String>,

    /// Skew metric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew: Option<u64>,

    /// State or region name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Spur IP Geo location information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Location {
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Country code (ISO 3166-1 alpha-2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Latitude coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// State or region name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Information about tunneling methods (VPN, TOR, etc.) used.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tunnel {
    /// Whether this tunnel is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,

    /// List of tunnel entries (ingress points).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<TunnelEntry>>,

    /// The operator or service running this tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,

    /// Type of tunnel (VPN, Proxy, Tor).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tunnel_type: Option<TunnelType>,
}

/// A tunnel entry (ingress point).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TunnelEntry {
    /// IP address of the entry point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Location of the entry point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Autonomous system of the entry point.
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub autonomous_system: Option<AutonomousSystem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_full_context() {
        let json = r#"{
            "as": {
                "number": 49981,
                "organization": "WorldStream"
            },
            "client": {
                "behaviors": ["FILE_SHARING", "TOR_PROXY_USER"],
                "concentration": {
                    "city": "Polāia Kalān",
                    "country": "IN",
                    "density": 0.2675,
                    "geohash": "tsn",
                    "skew": 6762,
                    "state": "Madhya Pradesh"
                },
                "count": 4,
                "countries": 2,
                "proxies": ["ABCPROXY_PROXY", "9PROXY_PROXY", "NETNUT_PROXY", "GOPROXY_PROXY"],
                "spread": 4724209,
                "types": ["MOBILE", "DESKTOP"]
            },
            "infrastructure": "DATACENTER",
            "ip": "89.39.106.191"
        }"#;

        let context: IpContext = serde_json::from_str(json).unwrap();

        assert_eq!(context.ip.as_deref(), Some("89.39.106.191"));
        assert_eq!(context.infrastructure, Some(Infrastructure::Datacenter));

        let asys = context.autonomous_system.as_ref().unwrap();
        assert_eq!(asys.number, Some(49981));
        assert_eq!(asys.organization.as_deref(), Some("WorldStream"));

        let client = context.client.as_ref().unwrap();
        assert_eq!(client.count, Some(4));
        assert_eq!(client.countries, Some(2));
        assert_eq!(
            client.behaviors.as_ref().unwrap(),
            &vec![Behavior::FileSharing, Behavior::TorProxyUser]
        );
        assert_eq!(
            client.types.as_ref().unwrap(),
            &vec![DeviceType::Mobile, DeviceType::Desktop]
        );

        let conc = client.concentration.as_ref().unwrap();
        assert_eq!(conc.country.as_deref(), Some("IN"));
        assert_eq!(conc.density, Some(0.2675));
    }

    #[test]
    fn test_deserialize_empty_context() {
        let json = "{}";
        let context: IpContext = serde_json::from_str(json).unwrap();
        assert!(context.ip.is_none());
        assert!(context.autonomous_system.is_none());
        assert!(context.client.is_none());
    }

    #[test]
    fn test_deserialize_minimal_context() {
        let json = r#"{"ip": "1.2.3.4"}"#;
        let context: IpContext = serde_json::from_str(json).unwrap();
        assert_eq!(context.ip.as_deref(), Some("1.2.3.4"));
    }

    #[test]
    fn test_deserialize_with_tunnels() {
        let json = r#"{
            "ip": "1.2.3.4",
            "tunnels": [
                {
                    "type": "VPN",
                    "operator": "NordVPN",
                    "anonymous": true,
                    "entries": [
                        {
                            "ip": "5.6.7.8",
                            "location": {
                                "city": "Amsterdam",
                                "country": "NL"
                            }
                        }
                    ]
                }
            ]
        }"#;

        let context: IpContext = serde_json::from_str(json).unwrap();
        let tunnels = context.tunnels.as_ref().unwrap();
        assert_eq!(tunnels.len(), 1);
        assert_eq!(tunnels[0].tunnel_type, Some(TunnelType::Vpn));
        assert_eq!(tunnels[0].operator.as_deref(), Some("NordVPN"));
        assert_eq!(tunnels[0].anonymous, Some(true));

        let entries = tunnels[0].entries.as_ref().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].ip.as_deref(), Some("5.6.7.8"));
    }

    #[test]
    fn test_serialize_context() {
        let context = IpContext {
            ip: Some("1.2.3.4".to_string()),
            infrastructure: Some(Infrastructure::Residential),
            ..Default::default()
        };

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains(r#""ip":"1.2.3.4""#));
        assert!(json.contains(r#""infrastructure":"RESIDENTIAL""#));
        // None fields should not be serialized
        assert!(!json.contains("client"));
        assert!(!json.contains("tunnels"));
    }

    #[test]
    fn test_deserialize_with_ai() {
        let json = r#"{
            "ip": "1.2.3.4",
            "ai": {
                "scrapers": true,
                "bots": false,
                "services": ["OPENAI", "ANTHROPIC"]
            }
        }"#;

        let context: IpContext = serde_json::from_str(json).unwrap();
        let ai = context.ai.as_ref().unwrap();
        assert_eq!(ai.scrapers, Some(true));
        assert_eq!(ai.bots, Some(false));
        assert_eq!(ai.services.as_ref().unwrap(), &vec!["OPENAI", "ANTHROPIC"]);
    }
}
