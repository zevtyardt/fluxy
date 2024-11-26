use std::net::Ipv4Addr;

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
}

#[derive(Debug)]
pub struct Proxy {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub anonymity: Anonymity,
    pub protocols: Vec<Protocol>,
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            port: 0,
            anonymity: Anonymity::Unknown,
            protocols: vec![],
        }
    }
}
