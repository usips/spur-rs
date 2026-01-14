//! Strongly typed enums for Spur API fields.
//!
//! All enums include an `Other(String)` variant for forward compatibility
//! with API additions. Unknown values deserialize to `Other` rather than
//! causing errors.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Macro for implementing serde traits on enums with an Other variant.
macro_rules! impl_serde_enum {
    ($enum_name:ident { $($variant:ident => $str:literal),+ $(,)? }) => {
        impl Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s = match self {
                    $(Self::$variant => $str,)+
                    Self::Other(s) => s.as_str(),
                };
                serializer.serialize_str(s)
            }
        }

        impl<'de> Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(match s.as_str() {
                    $($str => Self::$variant,)+
                    _ => Self::Other(s),
                })
            }
        }

        impl fmt::Display for $enum_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $str),)+
                    Self::Other(s) => write!(f, "{}", s),
                }
            }
        }

        impl $enum_name {
            /// Returns the string representation of this variant.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $str,)+
                    Self::Other(s) => s.as_str(),
                }
            }

            /// Returns true if this is an `Other` variant.
            pub fn is_other(&self) -> bool {
                matches!(self, Self::Other(_))
            }
        }
    };
}

/// Infrastructure type classification for an IP address.
///
/// Indicates the type of network the IP belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Infrastructure {
    /// IP from a datacenter or cloud provider.
    Datacenter,
    /// Home ISP connection.
    Residential,
    /// Mobile carrier network.
    Mobile,
    /// Business or enterprise network.
    Business,
    /// Unknown infrastructure type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(Infrastructure {
    Datacenter => "DATACENTER",
    Residential => "RESIDENTIAL",
    Mobile => "MOBILE",
    Business => "BUSINESS",
});

impl Default for Infrastructure {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

/// Risk factors or suspicious behaviors identified for an IP.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Risk {
    /// IP is associated with tunneling/proxy activity.
    Tunnel,
    /// IP has been associated with spam.
    Spam,
    /// IP is used as a callback proxy.
    CallbackProxy,
    /// Geographic location mismatch detected.
    GeoMismatch,
    /// Unknown risk type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(Risk {
    Tunnel => "TUNNEL",
    Spam => "SPAM",
    CallbackProxy => "CALLBACK_PROXY",
    GeoMismatch => "GEO_MISMATCH",
});

impl Default for Risk {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

/// Network services or protocols detected on an IP.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Service {
    /// OpenVPN protocol.
    OpenVpn,
    /// IPSec protocol.
    Ipsec,
    /// WireGuard protocol.
    Wireguard,
    /// SSH tunneling.
    Ssh,
    /// PPTP protocol.
    Pptp,
    /// Unknown service type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(Service {
    OpenVpn => "OPENVPN",
    Ipsec => "IPSEC",
    Wireguard => "WIREGUARD",
    Ssh => "SSH",
    Pptp => "PPTP",
});

impl Default for Service {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

/// Type of tunnel used for traffic anonymization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TunnelType {
    /// Virtual Private Network.
    Vpn,
    /// Proxy server.
    Proxy,
    /// The Onion Router network.
    Tor,
    /// Unknown tunnel type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(TunnelType {
    Vpn => "VPN",
    Proxy => "PROXY",
    Tor => "TOR",
});

impl Default for TunnelType {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

/// Client behavior patterns observed from an IP.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Behavior {
    /// File sharing activity (P2P, torrents, etc.).
    FileSharing,
    /// User of Tor or proxy services.
    TorProxyUser,
    /// Unknown behavior type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(Behavior {
    FileSharing => "FILE_SHARING",
    TorProxyUser => "TOR_PROXY_USER",
});

impl Default for Behavior {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

/// Device type classification for clients behind an IP.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Mobile device (phone, tablet).
    Mobile,
    /// Desktop or laptop computer.
    Desktop,
    /// Unknown device type not yet defined in this library.
    Other(String),
}

impl_serde_enum!(DeviceType {
    Mobile => "MOBILE",
    Desktop => "DESKTOP",
});

impl Default for DeviceType {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infrastructure_serde() {
        // Known variant
        let infra = Infrastructure::Datacenter;
        let json = serde_json::to_string(&infra).unwrap();
        assert_eq!(json, r#""DATACENTER""#);

        let parsed: Infrastructure = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Infrastructure::Datacenter);

        // Unknown variant
        let json = r#""SATELLITE""#;
        let parsed: Infrastructure = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, Infrastructure::Other("SATELLITE".to_string()));
        assert!(parsed.is_other());
    }

    #[test]
    fn test_risk_serde() {
        let risk = Risk::CallbackProxy;
        let json = serde_json::to_string(&risk).unwrap();
        assert_eq!(json, r#""CALLBACK_PROXY""#);

        let parsed: Risk = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Risk::CallbackProxy);

        // Unknown risk
        let json = r#""NEW_RISK_TYPE""#;
        let parsed: Risk = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, Risk::Other("NEW_RISK_TYPE".to_string()));
    }

    #[test]
    fn test_service_serde() {
        let service = Service::OpenVpn;
        let json = serde_json::to_string(&service).unwrap();
        assert_eq!(json, r#""OPENVPN""#);

        let parsed: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Service::OpenVpn);
    }

    #[test]
    fn test_tunnel_type_serde() {
        let tunnel = TunnelType::Tor;
        let json = serde_json::to_string(&tunnel).unwrap();
        assert_eq!(json, r#""TOR""#);

        let parsed: TunnelType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, TunnelType::Tor);
    }

    #[test]
    fn test_behavior_serde() {
        let behavior = Behavior::FileSharing;
        let json = serde_json::to_string(&behavior).unwrap();
        assert_eq!(json, r#""FILE_SHARING""#);

        let parsed: Behavior = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Behavior::FileSharing);
    }

    #[test]
    fn test_device_type_serde() {
        let device = DeviceType::Desktop;
        let json = serde_json::to_string(&device).unwrap();
        assert_eq!(json, r#""DESKTOP""#);

        let parsed: DeviceType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, DeviceType::Desktop);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Infrastructure::Datacenter.as_str(), "DATACENTER");
        assert_eq!(
            Infrastructure::Other("CUSTOM".to_string()).as_str(),
            "CUSTOM"
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Risk::Tunnel), "TUNNEL");
        assert_eq!(format!("{}", Risk::Other("CUSTOM".to_string())), "CUSTOM");
    }
}
