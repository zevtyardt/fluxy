use std::{fmt::Display, net::Ipv4Addr, str::FromStr, sync::Arc, time::Duration};

use hyper::Uri;

/// Represents the level of anonymity of a proxy.
#[derive(Debug, PartialEq)]
pub enum Anonymity {
    /// Elite anonymity: No IP address or headers are leaked.
    Elite,
    /// Transparent anonymity: Original IP address is visible.
    Transparent,
    /// Anonymous anonymity: Some headers may be leaked, but IP is hidden.
    Anonymous,
    Unknown,
}

/// Represents different protocols that a proxy can support.
#[derive(Debug, PartialEq)]
pub enum Protocol {
    Http(Anonymity),
    Https,
    Socks4,
    Socks5,
    Connect(u16),
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(anonimity) => match anonimity {
                Anonymity::Unknown => write!(f, "HTTP"),
                Anonymity::Elite => write!(f, "HTTP: Elite"),
                Anonymity::Transparent => write!(f, "HTTP: Transparent"),
                Anonymity::Anonymous => write!(f, "HTTP: Anonymous"),
            },
            Self::Https => write!(f, "HTTPS"),
            Self::Socks4 => write!(f, "SOCKS4"),
            Self::Socks5 => write!(f, "SOCKS5"),
            Self::Connect(port) => write!(f, "CONNECT:{}", port),
        }
    }
}

/// Contains geographical data related to a proxy.
#[derive(Debug, Default, Clone)]
pub struct GeoData {
    pub iso_code: Option<String>,
    pub name: Option<String>,
    pub region_iso_code: Option<String>,
    pub region_name: Option<String>,
    pub city_name: Option<String>,
}

/// Represents a proxy with its details.
#[derive(Debug, Clone)]
pub struct Proxy {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub geo: GeoData,
    pub runtimes: Vec<f64>,
    pub types: Vec<Arc<Protocol>>,
}

impl Proxy {
    /// Calculate the average proxy response time
    pub fn avg_response_time(&self) -> f64 {
        if self.runtimes.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.runtimes.iter().sum();
        sum / self.runtimes.len() as f64
    }

    /// Return proxy in <ip>:<port> format
    pub fn as_text(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            port: 0,
            geo: GeoData::default(),
            runtimes: vec![],
            types: vec![],
        }
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(iso_code) = &self.geo.iso_code {
            write!(f, "<Proxy {}", iso_code)?;
        } else {
            write!(f, "<Proxy --")?;
        }

        write!(
            f,
            " {:.2}s [{}] {}:{}>",
            self.avg_response_time(),
            self.types
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            self.ip,
            self.port
        )
    }
}

/// Represents a source of proxy information, such as a URL and default protocol types.
pub struct Source {
    /// URL of the proxy source.
    pub url: Uri,
    /// Default protocol types for the source.
    pub default_types: Vec<Arc<Protocol>>,
    /// Time before giving up.
    pub timeout: Duration,
}

impl Source {
    /// Creates a new `Source` with a specified URL and protocol types.
    /// If no types are provided, defaults to common protocols.
    pub fn new(url: &str, types: Vec<Protocol>) -> Self {
        let types = if types.is_empty() {
            vec![
                Arc::new(Protocol::Http(Anonymity::Unknown)),
                Arc::new(Protocol::Https),
                Arc::new(Protocol::Socks4),
                Arc::new(Protocol::Socks5),
                Arc::new(Protocol::Connect(25)),
                Arc::new(Protocol::Connect(80)),
            ]
        } else {
            types.into_iter().map(Arc::new).collect()
        };

        Self {
            url: Uri::from_str(url).unwrap(),
            default_types: types,
            timeout: Duration::from_secs(3),
        }
    }

    /// Creates a `Source` with default common protocols.
    pub fn all(url: &str) -> Self {
        Self::new(url, vec![])
    }

    /// Creates a `Source` with default types for HTTP protocols.
    pub fn http(url: &str) -> Self {
        Self::new(
            url,
            vec![
                Protocol::Http(Anonymity::Unknown),
                Protocol::Https,
                Protocol::Connect(80),
                Protocol::Connect(25),
            ],
        )
    }

    /// Creates a `Source` with default types for SOCKS protocols.
    pub fn socks(url: &str) -> Self {
        Self::new(url, vec![Protocol::Socks4, Protocol::Socks5])
    }
}

/// Options to filter proxy output
#[derive(Default)]
pub struct ProxyFilter {
    /// Filter proxies by ISO country code, if empty skip filtering. (optional)
    pub countries: Vec<String>,
    /// Filter proxies by protocol, if protocol is http ignore anonymity. if empty skip filtering (optional)
    pub types: Vec<Protocol>,
}

impl ProxyFilter {
    /// Ensure each proxy complies with the country's ISO code selection.
    pub fn is_country_match(&self, proxy: &Proxy) -> bool {
        if self.countries.is_empty() {
            return true;
        }
        proxy
            .geo
            .iso_code
            .as_ref()
            .map(|iso| self.countries.contains(iso))
            .unwrap_or(false)
    }

    /// Ensure each proxy complies with the selected protocol type.
    pub fn is_types_match(&self, proxy: &Proxy) -> bool {
        if self.types.is_empty() {
            return true;
        }
        proxy
            .types
            .iter()
            .any(|protocol| self.types.contains(protocol))
    }
}

/// Options for configuring the proxy fetching process.
pub struct ProxyFetcherConfig {
    /// Ensure each proxy has a unique IP; affects performance (default: true).
    pub enforce_unique_ip: bool,
    /// Maximum number of concurrent requests to process source URLs (default: 20).
    pub concurrency_limit: usize,
    /// Timeout for requests in milliseconds (default: 3000).
    pub request_timeout: u64,
    /// Perform geo lookup for each proxy; affects performance (default: true).
    pub enable_geo_lookup: bool,
    /// Filter proxies based on given option
    pub filters: ProxyFilter,
}

impl Default for ProxyFetcherConfig {
    fn default() -> Self {
        Self {
            enforce_unique_ip: true,
            concurrency_limit: 20,
            request_timeout: 3000,
            enable_geo_lookup: true,
            filters: ProxyFilter::default(),
        }
    }
}
