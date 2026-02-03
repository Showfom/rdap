# RDAP Rust Client

A modern, elegant RDAP (Registration Data Access Protocol) client written in Rust with beautiful colored output.

## Features

✨ **Modern & Fast**
- Asynchronous I/O with Tokio
- Efficient HTTP client with connection pooling
- Fast JSON parsing with Serde

🎨 **Beautiful Output**
- Colorized terminal output
- Pretty-printed tables
- Clear hierarchical display

🔍 **Full RDAP Support**
- Domain queries
- IP address queries (IPv4/IPv6)
- AS number queries
- Entity queries
- Nameserver queries
- Search queries

🚀 **Smart Features**
- Automatic bootstrap service discovery
- Query type auto-detection
- Multi-layer RDAP queries (registry + registrar for domains)
- Custom TLD overrides for ccTLDs not in IANA bootstrap
- Abuse contact display for IP/ASN queries
- Disk caching of bootstrap files
- Configurable timeouts

⚙️ **Configuration**
- Built-in defaults for zero-config usage
- Configurable bootstrap URLs and TLD overrides
- Support for local override files (*.local.json)
- `rdap update` command to fetch latest configs from GitHub

## Installation

### From Source

```bash
git clone https://github.com/Akaere-NetWorks/rdap.git
cd rdap
cargo build --release
sudo cp target/release/rdap /usr/local/bin/
```

### Using Cargo

```bash
cargo install rdap
```

## Usage

### Basic Queries

```bash
# Query a domain
rdap example.com

# Query a TLD (top-level domain)
rdap google
rdap com
rdap io

# Query an IP address
rdap 192.0.2.1
rdap 2001:db8::1

# Query with shorthand IP (auto-normalized)
rdap 1.1          # → queries 1.0.0.1
rdap 8.8          # → queries 8.0.0.8

# Query a CIDR range
rdap 8.8.8.0/24

# Query an AS number
rdap AS15169
rdap 15169

# Query with verbose output
rdap -v example.com
```

### Advanced Options

```bash
# Specify query type explicitly
rdap -t domain example.com

# Use a specific RDAP server
rdap -s https://rdap.verisign.com/com/v1 example.com

# JSON output
rdap -f json example.com
rdap -f json-pretty example.com

# Set custom timeout (in seconds)
rdap --timeout 60 example.com

# Disable registrar referral following for domain queries
rdap --no-referral example.com

# Update configuration files from GitHub
rdap --update
rdap -u
```

### Output Formats

- `text` - Beautiful colored terminal output (default)
- `json` - Compact JSON
- `json-pretty` - Pretty-printed JSON

## Examples

### Domain Query

```bash
$ rdap example.com

Domain Name: EXAMPLE.COM
Handle: 2336799_DOMAIN_COM-VRSN
Object Class: domain
Status: client delete prohibited
Status: client transfer prohibited
Status: client update prohibited
Nameserver: A.IANA-SERVERS.NET
Nameserver: B.IANA-SERVERS.NET
Delegation Signed: yes
DS Key Tag: 370
DS Algorithm: 13
DS Digest Type: 2
DS Digest: BE74359954660069D5C63D200C39F5603827D7DD02B56F120EE9F3A86764247C
Registration: 1995-08-14T04:00:00Z
Expiration: 2026-08-13T04:00:00Z
Last Changed: 2025-08-14T07:01:39Z
Last Update: 2025-11-04T20:54:25Z

Entity Handle: 376
Role: registrar
Name: RESERVED-Internet Assigned Numbers Authority
IANA Registrar ID: 376
```

### TLD Query

```bash
$ rdap google

Domain Name: google
Object Class: domain
Status: active
Nameserver: ns-tld1.charlestonroadregistry.com (216.239.32.105, 2001:4860:4802:32::69)
Nameserver: ns-tld2.charlestonroadregistry.com (216.239.34.105, 2001:4860:4802:34::69)
...
Delegation Signed: yes
Registration: 2014-09-04T00:00:00+00:00

Role: registrant
Name: Charleston Road Registry Inc.
Address: 1600 Amphitheatre Parkway
Mountain View CA 94043
United States of America (the)
```

<<<<<<< HEAD
### IP Query (with CIDR support)

```bash
$ rdap 8.8.8.0/24

Abuse contact for `8.8.8.0/24` is `network-abuse@google.com`

Query from https://rdap.arin.net/registry/ip/8.8.8.0/24

Handle: NET-8-8-8-0-2
Start Address: 8.8.8.0
End Address: 8.8.8.255
IP Version: v4
Name: GOGL
...
```
=======
### IP Query
>>>>>>> a367f2a4778627b8efe8d658b31c5f732bdf77da

```bash
$ rdap 8.8.8.8

Handle: NET-8-8-8-0-2
Start Address: 8.8.8.0
End Address: 8.8.8.255
IP Version: v4
Name: GOGL
Type: DIRECT ALLOCATION
Parent Handle: NET-8-0-0-0-0
Status: active
Port43: whois.arin.net
last changed: 2023-12-28T17:24:56-05:00
registration: 2023-12-28T17:24:33-05:00

Entity Handle: GOGL
Role: registrant
Name: Google LLC
Port43: whois.arin.net
last changed: 2019-10-31T15:45:45-04:00
registration: 2000-03-30T00:00:00-05:00
```

### AS Number Query

```bash
$ rdap AS213605

AS Number: AS213605
Name: Pysio-Research-NetWork
Handle: AS213605
Object Class: autnum
Status: active
Port43: whois.ripe.net
Registration: 2025-01-10T12:53:39Z
Last Changed: 2025-10-14T13:21:47Z

Entity Handle: LA9082-RIPE
Role: administrative
Role: technical
Role: abuse
Name: LiuHaoRan
Email: team@pysio.online
Phone: +86 19934273163
Link: https://rdap.db.ripe.net/entity/LA9082-RIPE
```

### Entity Query

```bash
$ rdap -s https://rdap.db.ripe.net -t entity LA9082-RIPE

Entity Handle: LA9082-RIPE
Role: administrative
Role: technical
Role: abuse
Name: LiuHaoRan
Email: team@pysio.online
Phone: +86 19934273163
Port43: whois.ripe.net
registration: 2020-01-15T10:30:00Z
last changed: 2025-01-06T08:29:19Z
Link: https://rdap.db.ripe.net/entity/LA9082-RIPE
```

### Verbose Output

```bash
$ rdap -v AS213605

→ Query: AS213605
→ Type:  autnum

⟳ Querying RDAP server...

AS Number: AS213605
Name: Pysio-Research-NetWork
Handle: AS213605
Object Class: autnum
Status: active
Port43: whois.ripe.net
Registration: 2025-01-10T12:53:39Z
Last Changed: 2025-10-14T13:21:47Z

Entity Handle: LA9082-RIPE
Role: administrative
Role: technical
Role: abuse
Name: LiuHaoRan
Email: team@pysio.online
Phone: +86 19934273163
Link: https://rdap.db.ripe.net/entity/LA9082-RIPE
Link: http://www.ripe.net/data-tools/support/documentation/terms (copyright)

Notice: Filtered
  This output has been filtered.
Notice: Source
  Objects returned came from source
  RIPE
Notice: Terms and Conditions
  This is the RIPE Database query service. The objects are in RDAP format.
  Link: http://www.ripe.net/db/support/db-terms-conditions.pdf
```

## Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rdap = { git = "https://github.com/Akaere-NetWorks/rdap.git" }
tokio = { version = "1.35", features = ["full"] }
```

Or use a specific version/branch:

```toml
[dependencies]
# Use main branch
rdap = { git = "https://github.com/Akaere-NetWorks/rdap.git", branch = "main" }

# Or use a specific tag (when available)
# rdap = { git = "https://github.com/Akaere-NetWorks/rdap.git", tag = "v0.1.0" }

# Or use a specific commit
# rdap = { git = "https://github.com/Akaere-NetWorks/rdap.git", rev = "abc123" }

tokio = { version = "1.35", features = ["full"] }
```

### Basic Query

```rust
use rdap::{RdapClient, RdapRequest, QueryType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = RdapClient::new()?;
    
    // Query a domain
    let request = RdapRequest::new(QueryType::Domain, "example.com");
    let result = client.query(&request).await?;
    
    // Display with colored output
    use rdap::display::RdapDisplay;
    result.display(false); // false = non-verbose
    
    Ok(())
}
```

### Auto-Detection

```rust
use rdap::{RdapClient, RdapRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    
    // Auto-detect query type
    let query = "8.8.8.8";
    let query_type = RdapRequest::detect_type(query)?;
    
    let request = RdapRequest::new(query_type, query);
    let result = client.query(&request).await?;
    
    // Process the result based on type
    match result {
        rdap::RdapObject::Domain(domain) => {
            println!("Domain: {}", domain.ldh_name.unwrap_or_default());
        }
        rdap::RdapObject::IpNetwork(ip) => {
            println!("IP Network: {}", ip.name.unwrap_or_default());
        }
        rdap::RdapObject::Autnum(asn) => {
            println!("AS Number: AS{}", asn.start_autnum.unwrap_or(0));
        }
        _ => {}
    }
    
    Ok(())
}
```

### Custom Server

```rust
use rdap::{RdapClient, RdapRequest, QueryType};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    
    // Use a specific RDAP server
    let server = Url::parse("https://rdap.verisign.com/com/v1")?;
    let request = RdapRequest::new(QueryType::Domain, "example.com")
        .with_server(server);
    
    let result = client.query(&request).await?;
    
    Ok(())
}
```

### JSON Output

```rust
use rdap::{RdapClient, RdapRequest, QueryType};
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    let request = RdapRequest::new(QueryType::Domain, "example.com");
    let result = client.query(&request).await?;
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&result)?;
    println!("{}", json);
    
    Ok(())
}
```

### Working with Domain Data

```rust
use rdap::{RdapClient, RdapRequest, QueryType, RdapObject};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    let request = RdapRequest::new(QueryType::Domain, "example.com");
    let result = client.query(&request).await?;
    
    if let RdapObject::Domain(domain) = result {
        // Access domain properties
        println!("Domain: {}", domain.ldh_name.unwrap_or_default());
        
        // Check status
        for status in &domain.status {
            println!("Status: {}", status);
        }
        
        // List nameservers
        for ns in &domain.nameservers {
            if let Some(name) = &ns.ldh_name {
                println!("Nameserver: {}", name);
            }
        }
        
        // Check DNSSEC
        if let Some(dnssec) = &domain.secure_dns {
            if let Some(signed) = dnssec.delegation_signed {
                println!("DNSSEC: {}", if signed { "Signed" } else { "Not signed" });
            }
        }
        
        // Access entities (registrar, registrant, etc.)
        for entity in &domain.entities {
            if let Some(handle) = &entity.handle {
                println!("Entity: {} (roles: {:?})", handle, entity.roles);
            }
            
            // Access vCard data
            if let Some(vcard) = &entity.vcard {
                if let Some(name) = vcard.name() {
                    println!("  Name: {}", name);
                }
                if let Some(email) = vcard.email() {
                    println!("  Email: {}", email);
                }
            }
        }
        
        // Access events
        for event in &domain.events {
            println!("Event: {} at {}", event.action, event.date);
        }
    }
    
    Ok(())
}
```

### Working with IP Network Data

```rust
use rdap::{RdapClient, RdapRequest, QueryType, RdapObject};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    let request = RdapRequest::new(QueryType::Ip, "8.8.8.8");
    let result = client.query(&request).await?;
    
    if let RdapObject::IpNetwork(network) = result {
        println!("Network: {}", network.name.unwrap_or_default());
        println!("Range: {} - {}", 
            network.start_address.unwrap_or_default(),
            network.end_address.unwrap_or_default()
        );
        println!("Type: {}", network.network_type.unwrap_or_default());
        
        if let Some(country) = &network.country {
            println!("Country: {}", country);
        }
    }
    
    Ok(())
}
```

### Working with AS Number Data

```rust
use rdap::{RdapClient, RdapRequest, QueryType, RdapObject};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RdapClient::new()?;
    let request = RdapRequest::new(QueryType::Autnum, "AS15169");
    let result = client.query(&request).await?;
    
    if let RdapObject::Autnum(asn) = result {
        if let Some(start) = asn.start_autnum {
            println!("AS Number: AS{}", start);
        }
        println!("Name: {}", asn.name.unwrap_or_default());
        println!("Type: {}", asn.as_type.unwrap_or_default());
        
        if let Some(country) = &asn.country {
            println!("Country: {}", country);
        }
    }
    
    Ok(())
}
```

### Error Handling

```rust
use rdap::{RdapClient, RdapRequest, QueryType, RdapObject, RdapError};

#[tokio::main]
async fn main() {
    let client = RdapClient::new().unwrap();
    let request = RdapRequest::new(QueryType::Domain, "example.com");
    
    match client.query(&request).await {
        Ok(result) => {
            // Handle successful response
            match result {
                RdapObject::Error(err) => {
                    eprintln!("RDAP Error: {}", err.title.unwrap_or_default());
                    for desc in &err.description {
                        eprintln!("  {}", desc);
                    }
                }
                _ => {
                    use rdap::display::RdapDisplay;
                    result.display(false);
                }
            }
        }
        Err(e) => {
            match e {
                RdapError::Bootstrap(msg) => {
                    eprintln!("Bootstrap error: {}", msg);
                }
                RdapError::Http(err) => {
                    eprintln!("HTTP error: {}", err);
                }
                RdapError::InvalidQuery(msg) => {
                    eprintln!("Invalid query: {}", msg);
                }
                _ => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
```

### Advanced: Custom Timeout and Configuration

```rust
use rdap::{RdapClient, RdapRequest, QueryType};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with custom timeout
    let client = RdapClient::new()?
        .with_timeout(Duration::from_secs(30));
    
    let request = RdapRequest::new(QueryType::Domain, "example.com");
    let result = client.query(&request).await?;
    
    Ok(())
}
```

### Integration Example: Web Service

Here's an example of using the RDAP library in a web service:

```rust
use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use rdap::{RdapClient, RdapRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

// Shared RDAP client
struct AppState {
    rdap_client: Arc<Mutex<RdapClient>>,
}

#[tokio::main]
async fn main() {
    let client = RdapClient::new().unwrap();
    let state = Arc::new(AppState {
        rdap_client: Arc::new(Mutex::new(client)),
    });

    let app = Router::new()
        .route("/rdap/:query", get(query_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn query_handler(
    Path(query): Path<String>,
    state: axum::extract::State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state.rdap_client.lock().await;
    
    // Auto-detect query type
    let query_type = RdapRequest::detect_type(&query)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let request = RdapRequest::new(query_type, &query);
    
    match client.query(&request).await {
        Ok(result) => {
            let json = serde_json::to_value(&result)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(json))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
```

## Configuration

The RDAP client uses a configuration system with the following priority (highest to lowest):

1. `~/.config/rdap/*.local.json` - User local overrides (never overwritten by updates)
2. `~/.config/rdap/*.json` - Downloaded configs (updated via `rdap update`)
3. `/etc/rdap/*.json` - System-wide configs
4. Built-in defaults (embedded in binary)

### Configuration Files

- **config.json** - Bootstrap URLs for IANA RDAP service discovery
- **tlds.json** - TLD overrides for ccTLDs not in IANA bootstrap
- **config.local.json** - Your custom bootstrap config (optional, survives updates)
- **tlds.local.json** - Your custom TLD overrides (merged on top of tlds.json)

### Updating Configs

```bash
# Update configs from GitHub
rdap --update
rdap -u
```

This downloads:
- `config.json` and `tlds.json` from the GitHub repository
- `tlds.txt` (IANA TLD list) from https://data.iana.org/TLD/tlds-alpha-by-domain.txt

Your `*.local.json` files are preserved.

### Custom TLD Overrides

Create `~/.config/rdap/tlds.local.json` to add your own TLD overrides:

```json
{
  "example": "https://rdap.example.com/",
  "co.example": "https://rdap.co.example.com/"
}
```

These will be merged on top of the base `tlds.json` configuration.

## Architecture

```
src/
├── lib.rs           # Library entry point
├── main.rs          # CLI entry point
├── error.rs         # Error types
├── config.rs        # Configuration management
├── models/          # RDAP data models
│   ├── domain.rs
│   ├── entity.rs
│   ├── autnum.rs
│   ├── ip_network.rs
│   ├── nameserver.rs
│   └── ...
├── client.rs        # RDAP client
├── request.rs       # Request builder
├── bootstrap.rs     # Bootstrap service discovery
├── cache.rs         # Bootstrap cache
└── display.rs       # Pretty output formatting

config/
├── config.json      # Default bootstrap URLs
├── tlds.json        # Default TLD overrides for ccTLDs
└── tlds.txt         # IANA TLD list for TLD query detection
```

## RFCs Implemented

- [RFC 7480](https://tools.ietf.org/html/rfc7480) - HTTP Usage in RDAP
- [RFC 7482](https://tools.ietf.org/html/rfc7482) - RDAP Query Format
- [RFC 7483](https://tools.ietf.org/html/rfc7483) - JSON Responses for RDAP
- [RFC 7484](https://tools.ietf.org/html/rfc7484) - Finding the Authoritative RDAP Service
- [RFC 6350](https://tools.ietf.org/html/rfc6350) - vCard Format
- [RFC 7095](https://tools.ietf.org/html/rfc7095) - jCard

## Comparison with Go Version

| Feature | Go Version | Rust Version |
|---------|-----------|--------------|
| Performance | ⚡ Fast | ⚡⚡ Very Fast |
| Memory Usage | Good | Excellent |
| Colored Output | Basic | Advanced |
| Table Formatting | None | Beautiful |
| Async Support | Yes | Yes (Tokio) |
| Type Safety | Runtime | Compile-time |
| Binary Size | ~8MB | ~4MB (stripped) |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Author

Akaere Networks

## Links

- Original Go version: https://github.com/openrdap/rdap
- IANA RDAP Bootstrap: https://data.iana.org/rdap/
- RDAP Working Group: https://datatracker.ietf.org/wg/weirds/
