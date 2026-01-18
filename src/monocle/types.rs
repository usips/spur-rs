//! Monocle assessment types.

use serde::{Deserialize, Serialize};

/// Decrypted Monocle assessment result.
///
/// This is the response from the Monocle Decryption API after decrypting
/// an encrypted assessment bundle from the client-side JavaScript.
///
/// ## API Endpoint
///
/// ```text
/// POST https://decrypt.mcl.spur.us/api/v1/assessment
/// Content-Type: text/plain; charset=utf-8
/// TOKEN: <MONOCLE_SECRET_KEY>
///
/// <encrypted_bundle>
/// ```
///
/// ## Example
///
/// ```rust
/// use spur::monocle::Assessment;
///
/// let json = r#"{
///     "vpn": true,
///     "proxied": false,
///     "anon": true,
///     "ip": "37.19.221.165",
///     "ts": "2022-12-01T01:00:50Z",
///     "complete": true,
///     "id": "0a3e401a-b0d5-496b-b1ff-6cb8eca542a2",
///     "sid": "example-form"
/// }"#;
///
/// let assessment: Assessment = serde_json::from_str(json).unwrap();
///
/// if assessment.vpn || assessment.proxied {
///     println!("User is using anonymization: {}", assessment.ip);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assessment {
    /// Whether a VPN was detected.
    ///
    /// This indicates the user is connected through a known VPN service.
    pub vpn: bool,

    /// Whether the traffic was proxied.
    ///
    /// This indicates the user is connected through a proxy service,
    /// which may include datacenter proxies or residential proxy networks.
    pub proxied: bool,

    /// Whether the connection is anonymous.
    ///
    /// This is a combined indicator that considers multiple anonymization
    /// factors including VPN usage, proxy usage, and other signals.
    pub anon: bool,

    /// The detected IP address of the client.
    ///
    /// This is the IP address that Monocle observed for this assessment.
    pub ip: String,

    /// Timestamp of the assessment (ISO 8601 format).
    ///
    /// Example: `"2022-12-01T01:00:50Z"`
    pub ts: String,

    /// Whether the assessment completed successfully.
    ///
    /// If `false`, the assessment may be incomplete and the results
    /// should be treated with caution.
    pub complete: bool,

    /// Unique assessment ID (UUID).
    ///
    /// This can be used for logging and debugging purposes.
    pub id: String,

    /// Session ID from the Monocle application.
    ///
    /// This corresponds to the session identifier configured in your
    /// Monocle JavaScript integration.
    pub sid: String,
}

impl Assessment {
    /// Returns `true` if any anonymization was detected.
    ///
    /// This is a convenience method that checks if the user appears to be
    /// using any form of anonymization (VPN, proxy, or other).
    ///
    /// # Example
    ///
    /// ```rust
    /// use spur::monocle::Assessment;
    ///
    /// let json = r#"{
    ///     "vpn": true, "proxied": false, "anon": true,
    ///     "ip": "1.2.3.4", "ts": "2022-12-01T00:00:00Z",
    ///     "complete": true, "id": "abc", "sid": "form"
    /// }"#;
    /// let assessment: Assessment = serde_json::from_str(json).unwrap();
    ///
    /// assert!(assessment.is_anonymized());
    /// ```
    pub fn is_anonymized(&self) -> bool {
        self.vpn || self.proxied || self.anon
    }

    /// Returns `true` if this assessment can be trusted.
    ///
    /// An assessment is considered trustworthy if it completed successfully.
    /// Incomplete assessments may have partial or unreliable results.
    pub fn is_trustworthy(&self) -> bool {
        self.complete
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_assessment() {
        let json = r#"{
            "vpn": true,
            "proxied": false,
            "anon": true,
            "ip": "37.19.221.165",
            "ts": "2022-12-01T01:00:50Z",
            "complete": true,
            "id": "0a3e401a-b0d5-496b-b1ff-6cb8eca542a2",
            "sid": "example-form"
        }"#;

        let assessment: Assessment = serde_json::from_str(json).unwrap();

        assert!(assessment.vpn);
        assert!(!assessment.proxied);
        assert!(assessment.anon);
        assert_eq!(assessment.ip, "37.19.221.165");
        assert_eq!(assessment.ts, "2022-12-01T01:00:50Z");
        assert!(assessment.complete);
        assert_eq!(assessment.id, "0a3e401a-b0d5-496b-b1ff-6cb8eca542a2");
        assert_eq!(assessment.sid, "example-form");
    }

    #[test]
    fn test_deserialize_clean_assessment() {
        let json = r#"{
            "vpn": false,
            "proxied": false,
            "anon": false,
            "ip": "192.168.1.1",
            "ts": "2023-06-15T12:30:00Z",
            "complete": true,
            "id": "clean-user-id",
            "sid": "login-form"
        }"#;

        let assessment: Assessment = serde_json::from_str(json).unwrap();

        assert!(!assessment.vpn);
        assert!(!assessment.proxied);
        assert!(!assessment.anon);
        assert!(!assessment.is_anonymized());
        assert!(assessment.is_trustworthy());
    }

    #[test]
    fn test_deserialize_proxy_assessment() {
        let json = r#"{
            "vpn": false,
            "proxied": true,
            "anon": true,
            "ip": "45.33.32.156",
            "ts": "2023-06-15T12:30:00Z",
            "complete": true,
            "id": "proxy-user-id",
            "sid": "checkout"
        }"#;

        let assessment: Assessment = serde_json::from_str(json).unwrap();

        assert!(!assessment.vpn);
        assert!(assessment.proxied);
        assert!(assessment.anon);
        assert!(assessment.is_anonymized());
    }

    #[test]
    fn test_deserialize_incomplete_assessment() {
        let json = r#"{
            "vpn": false,
            "proxied": false,
            "anon": false,
            "ip": "10.0.0.1",
            "ts": "2023-06-15T12:30:00Z",
            "complete": false,
            "id": "incomplete-id",
            "sid": "form"
        }"#;

        let assessment: Assessment = serde_json::from_str(json).unwrap();

        assert!(!assessment.complete);
        assert!(!assessment.is_trustworthy());
    }

    #[test]
    fn test_serialize_assessment() {
        let assessment = Assessment {
            vpn: true,
            proxied: false,
            anon: true,
            ip: "1.2.3.4".to_string(),
            ts: "2023-01-01T00:00:00Z".to_string(),
            complete: true,
            id: "test-id".to_string(),
            sid: "test-session".to_string(),
        };

        let json = serde_json::to_string(&assessment).unwrap();

        assert!(json.contains(r#""vpn":true"#));
        assert!(json.contains(r#""proxied":false"#));
        assert!(json.contains(r#""anon":true"#));
        assert!(json.contains(r#""ip":"1.2.3.4""#));
        assert!(json.contains(r#""complete":true"#));
    }

    #[test]
    fn test_roundtrip() {
        let original = Assessment {
            vpn: true,
            proxied: true,
            anon: true,
            ip: "203.0.113.50".to_string(),
            ts: "2024-01-15T08:30:00Z".to_string(),
            complete: true,
            id: "roundtrip-test-id".to_string(),
            sid: "roundtrip-session".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Assessment = serde_json::from_str(&json).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_is_anonymized() {
        // VPN only
        let vpn_only = Assessment {
            vpn: true,
            proxied: false,
            anon: false,
            ip: "1.1.1.1".to_string(),
            ts: "2023-01-01T00:00:00Z".to_string(),
            complete: true,
            id: "id".to_string(),
            sid: "sid".to_string(),
        };
        assert!(vpn_only.is_anonymized());

        // Proxy only
        let proxy_only = Assessment {
            vpn: false,
            proxied: true,
            anon: false,
            ip: "1.1.1.1".to_string(),
            ts: "2023-01-01T00:00:00Z".to_string(),
            complete: true,
            id: "id".to_string(),
            sid: "sid".to_string(),
        };
        assert!(proxy_only.is_anonymized());

        // Anon only (edge case)
        let anon_only = Assessment {
            vpn: false,
            proxied: false,
            anon: true,
            ip: "1.1.1.1".to_string(),
            ts: "2023-01-01T00:00:00Z".to_string(),
            complete: true,
            id: "id".to_string(),
            sid: "sid".to_string(),
        };
        assert!(anon_only.is_anonymized());

        // Clean
        let clean = Assessment {
            vpn: false,
            proxied: false,
            anon: false,
            ip: "1.1.1.1".to_string(),
            ts: "2023-01-01T00:00:00Z".to_string(),
            complete: true,
            id: "id".to_string(),
            sid: "sid".to_string(),
        };
        assert!(!clean.is_anonymized());
    }
}
