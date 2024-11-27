use std::{fmt::Display, net::Ipv4Addr, sync::Arc};

use reqwest::Url;

#[derive(Debug)]
pub enum Anonymity {
    Elite,
    Transparent,
    Anonymous,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    Http,
    Https,
    Socks4,
    Socks5,
    Connect25,
    Connect80,
}

#[derive(Debug)]
pub enum Country {
    Unknown,
    Id(String),
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "--"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug)]
pub struct Proxy {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub country: Country,
    pub anonymity: Anonymity,
    pub protocols: Vec<Protocol>,
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            port: 0,
            country: Country::Unknown,
            anonymity: Anonymity::Unknown,
            protocols: vec![],
        }
    }
}

impl Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Proxy {} {}:{}>", self.country, self.ip, self.port)
    }
}

pub struct Source {
    pub url: Url,
    pub default_protocols: Vec<Protocol>,
}

impl Source {
    pub fn default(url: &str) -> Self {
        Self {
            url: Url::parse(url).unwrap(),
            default_protocols: vec![
                Protocol::Http,
                Protocol::Https,
                Protocol::Socks4,
                Protocol::Socks5,
                Protocol::Connect25,
                Protocol::Connect80,
            ],
        }
    }
}
