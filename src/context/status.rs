//! API Status types for the Spur Context API.

use serde::{Deserialize, Serialize};

/// The status of an API token.
///
/// All fields may be omitted if their value is null.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ApiStatus {
    /// Whether the API token is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The number of queries remaining in this billing cycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries_remaining: Option<u64>,

    /// The service tier for this token (e.g., "online").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_api_status() {
        let json = r#"{
            "active": true,
            "queriesRemaining": 49283,
            "serviceTier": "online"
        }"#;

        let status: ApiStatus = serde_json::from_str(json).unwrap();

        assert_eq!(status.active, Some(true));
        assert_eq!(status.queries_remaining, Some(49283));
        assert_eq!(status.service_tier.as_deref(), Some("online"));
    }

    #[test]
    fn test_deserialize_empty_status() {
        let json = "{}";
        let status: ApiStatus = serde_json::from_str(json).unwrap();
        assert!(status.active.is_none());
        assert!(status.queries_remaining.is_none());
        assert!(status.service_tier.is_none());
    }

    #[test]
    fn test_deserialize_partial_status() {
        let json = r#"{"active": false}"#;
        let status: ApiStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.active, Some(false));
        assert!(status.queries_remaining.is_none());
    }

    #[test]
    fn test_serialize_status() {
        let status = ApiStatus {
            active: Some(true),
            queries_remaining: Some(1000),
            service_tier: Some("enterprise".to_string()),
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""active":true"#));
        assert!(json.contains(r#""queriesRemaining":1000"#));
        assert!(json.contains(r#""serviceTier":"enterprise""#));
    }

    #[test]
    fn test_serialize_partial_status() {
        let status = ApiStatus {
            active: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains(r#""active":true"#));
        // None fields should not be serialized
        assert!(!json.contains("queriesRemaining"));
        assert!(!json.contains("serviceTier"));
    }
}
