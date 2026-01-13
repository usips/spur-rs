# Test Fixtures

This directory contains JSON responses from the Spur API for testing the parser against real data.

## Adding New Fixtures

Since the Spur API is paid, you can use the free tier to fetch individual IP results and save them here for testing.

### From the API

```bash
# Fetch an IP context and save as a fixture
curl -s "https://api.spur.us/v2/context/1.2.3.4" \
  -H "Token: YOUR_API_TOKEN" \
  | jq . > tests/fixtures/descriptive_name.json
```

### From the Web Interface

1. Look up an IP at [spur.us](https://spur.us)
2. Copy the JSON response from the developer tools Network tab
3. Save to `tests/fixtures/<descriptive_name>.json`

## Naming Convention

Use descriptive names that indicate the IP type:

| Pattern | Description | Example |
|---------|-------------|---------|
| `vpn_*.json` | VPN exit nodes | `vpn_nordvpn.json`, `vpn_expressvpn.json` |
| `tor_*.json` | Tor exit/relay nodes | `tor_exit_de.json`, `tor_relay.json` |
| `residential_*.json` | Residential ISP IPs | `residential_comcast.json` |
| `datacenter_*.json` | Datacenter/cloud IPs | `datacenter_aws.json` |
| `mobile_*.json` | Mobile carrier IPs | `mobile_verizon.json` |
| `ai_*.json` or `*_scraper*.json` | AI/bot traffic | `ai_openai.json` |
| `proxy_*.json` | Proxy services | `proxy_luminati.json` |

## Automatic Testing

All `.json` files in this directory are automatically tested by `tests/fixture_tests.rs`:

- **Parse validation**: Every file must parse as a valid `IpContext`
- **Round-trip**: Serialize → deserialize produces identical data
- **IP format**: IP addresses must be valid IPv4 or IPv6
- **Type-specific checks**: VPN fixtures must have tunnels, Tor fixtures must have Tor type, etc.

Run the fixture tests:

```bash
cargo test fixture --nocapture
```

## Current Fixtures

| File | Description |
|------|-------------|
| `vpn_response.json` | NordVPN datacenter exit node |
| `residential_response.json` | Comcast residential IP |
| `tor_response.json` | Tor exit node in Germany |
| `ai_scraper_response.json` | OpenAI scraper IP |

## Privacy Note

When adding fixtures from real lookups:

1. **Anonymize if needed**: Consider changing IPs to documentation ranges (192.0.2.x, 198.51.100.x, 203.0.113.x) if the actual IP isn't important
2. **Remove sensitive data**: Strip any data you don't want committed
3. **Use public IPs**: VPN exit nodes, Tor nodes, and datacenter IPs are generally fine to include

## Example Fixture

```json
{
    "ip": "89.39.106.191",
    "infrastructure": "DATACENTER",
    "organization": "WorldStream B.V.",
    "as": {
        "number": 49981,
        "organization": "WorldStream B.V."
    },
    "location": {
        "country": "NL",
        "city": "Amsterdam"
    },
    "risks": ["TUNNEL"],
    "tunnels": [
        {
            "type": "VPN",
            "operator": "NordVPN",
            "anonymous": true
        }
    ]
}
```
