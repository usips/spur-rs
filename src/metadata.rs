//! Tag Metadata Object types for the Spur Context API.

use serde::{Deserialize, Serialize};

/// The Tag Metadata Object includes analysis, statistics, and metrics for a service tag.
///
/// All fields may be omitted if their value is null.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TagMetadata {
    /// Whether the service supports or facilitates crypto-based payments or platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_crypto: Option<String>,

    /// Whether the service is available for free usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_free_access: Option<String>,

    /// Whether the service offers multi-hop or chaining functionalities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multihop: Option<String>,

    /// Whether the service permits torrent or P2P file-sharing traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_torrents: Option<String>,

    /// Indicates whether white-label or rebranded versions of the service exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_white_label: Option<String>,

    /// Product categories for bandwidth reselling and routing
    /// (e.g., "RESIDENTIAL_PROXY", "DATACENTER_PROXY", "MOBILE_PROXY", "ISP_PROXY").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// A free-text description of the service or entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the service or infrastructure primarily aims to anonymize user traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<String>,

    /// Whether the service includes callback or reverse-proxy functionalities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_callback_proxy: Option<String>,

    /// Whether the service or platform is oriented toward enterprise usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enterprise: Option<String>,

    /// Whether the service is currently inactive or defunct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inactive: Option<String>,

    /// Whether the service claims a 'no logging' policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_no_log: Option<String>,

    /// Metrics and statistics for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<TagMetrics>,

    /// Human-readable name of the service or entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Operating systems and environments supported by this service
    /// (e.g., "ROUTER").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,

    /// Protocols or services used for network traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    /// Unique identifier or tag for this service or entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Possible granularities for configuring a service exit or route
    /// (e.g., "CITY", "STATE", "COUNTRY", "ASN").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_types: Option<Vec<String>>,

    /// Primary website or homepage for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

/// Metrics and statistics for a tagged service.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TagMetrics {
    /// Average number of devices observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_device_count: Option<String>,

    /// Churn rate of IPs or users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub churn_rate: Option<String>,

    /// Number of distinct autonomous system numbers observed.
    #[serde(rename = "distinctASNs", skip_serializing_if = "Option::is_none")]
    pub distinct_asns: Option<String>,

    /// Number of distinct countries observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_countries: Option<String>,

    /// Number of distinct IP addresses observed.
    #[serde(rename = "distinctIPs", skip_serializing_if = "Option::is_none")]
    pub distinct_ips: Option<String>,

    /// Number of distinct ISPs observed.
    #[serde(rename = "distinctISPs", skip_serializing_if = "Option::is_none")]
    pub distinct_isps: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_tag_metadata() {
        let json = r#"{
            "allowsCrypto": "false",
            "allowsFreeAccess": "false",
            "allowsMultihop": "false",
            "allowsTorrents": "false",
            "allowsWhiteLabel": "true",
            "categories": ["RESIDENTIAL_PROXY", "DATACENTER_PROXY", "MOBILE_PROXY", "ISP_PROXY"],
            "description": "OxyLabs is the second largest proxy provider tracked.",
            "isAnonymous": "true",
            "isCallbackProxy": "true",
            "isEnterprise": "false",
            "isInactive": "false",
            "isNoLog": "true",
            "metrics": {
                "averageDeviceCount": "37.20332478669546",
                "churnRate": "0.08675012801772562",
                "distinctASNs": "25334",
                "distinctCountries": "235",
                "distinctIPs": "6367903",
                "distinctISPs": "67413"
            },
            "name": "Oxylabs",
            "platforms": ["ROUTER"],
            "protocols": [],
            "tag": "OXYLABS_PROXY",
            "targetingTypes": ["CITY", "STATE", "COUNTRY", "ASN"],
            "website": "https://oxylabs.io"
        }"#;

        let meta: TagMetadata = serde_json::from_str(json).unwrap();

        assert_eq!(meta.allows_crypto.as_deref(), Some("false"));
        assert_eq!(meta.allows_white_label.as_deref(), Some("true"));
        assert_eq!(meta.name.as_deref(), Some("Oxylabs"));
        assert_eq!(meta.tag.as_deref(), Some("OXYLABS_PROXY"));
        assert_eq!(meta.is_anonymous.as_deref(), Some("true"));
        assert_eq!(meta.website.as_deref(), Some("https://oxylabs.io"));

        let categories = meta.categories.as_ref().unwrap();
        assert_eq!(categories.len(), 4);
        assert!(categories.contains(&"RESIDENTIAL_PROXY".to_string()));

        let metrics = meta.metrics.as_ref().unwrap();
        assert_eq!(metrics.distinct_ips.as_deref(), Some("6367903"));
        assert_eq!(metrics.distinct_asns.as_deref(), Some("25334"));
        assert_eq!(metrics.distinct_countries.as_deref(), Some("235"));
    }

    #[test]
    fn test_deserialize_empty_metadata() {
        let json = "{}";
        let meta: TagMetadata = serde_json::from_str(json).unwrap();
        assert!(meta.name.is_none());
        assert!(meta.tag.is_none());
        assert!(meta.metrics.is_none());
    }

    #[test]
    fn test_deserialize_minimal_metadata() {
        let json = r#"{"tag": "SOME_PROXY", "name": "Some Proxy"}"#;
        let meta: TagMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.tag.as_deref(), Some("SOME_PROXY"));
        assert_eq!(meta.name.as_deref(), Some("Some Proxy"));
    }

    #[test]
    fn test_serialize_metadata() {
        let meta = TagMetadata {
            tag: Some("TEST_PROXY".to_string()),
            name: Some("Test Proxy".to_string()),
            is_anonymous: Some("true".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains(r#""tag":"TEST_PROXY""#));
        assert!(json.contains(r#""name":"Test Proxy""#));
        assert!(json.contains(r#""isAnonymous":"true""#));
        // None fields should not be serialized
        assert!(!json.contains("website"));
        assert!(!json.contains("metrics"));
    }

    #[test]
    fn test_deserialize_with_empty_protocols() {
        let json = r#"{
            "tag": "SOME_VPN",
            "protocols": []
        }"#;

        let meta: TagMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(meta.protocols.as_ref().unwrap().len(), 0);
    }
}
