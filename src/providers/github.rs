use std::{
    io::BufReader,
    net::Ipv4Addr,
    sync::{atomic::AtomicUsize, mpsc, Arc},
};

use async_trait::async_trait;
use scraper::Html;

use super::IProxyTrait;
use crate::models::{Anonymity, Protocol, Proxy, Source};

pub struct GithubRepoProvider;

impl GithubRepoProvider {
    fn githubusercontent(
        &self, username: &str, path: &str, branch: &str, filename: &str,
    ) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/{}",
            username, path, branch, filename
        )
    }
}

#[async_trait]
impl IProxyTrait for GithubRepoProvider {
    fn sources(&self) -> Vec<Source> {
        vec![
            Source::new(
                &self.githubusercontent("zevtyardt", "proxy-list", "main", "http.txt"),
                vec![
                    Protocol::Http(Anonymity::Unknown),
                    Protocol::Https,
                    Protocol::Connect(80),
                    Protocol::Connect(25),
                ],
            ),
            Source::new(
                &self.githubusercontent("zevtyardt", "proxy-list", "main", "socks4.txt"),
                vec![Protocol::Socks4],
            ),
            Source::new(
                &self.githubusercontent("zevtyardt", "proxy-list", "main", "socks5.txt"),
                vec![Protocol::Socks5],
            ),
        ]
    }

    async fn scrape(
        &self, html: Html, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>, default_protocols: Vec<Arc<Protocol>>,
    ) -> anyhow::Result<Vec<Source>> {
        for line in html.html().lines() {
            let mut splited = line.trim().split(':');
            if let Some(Ok(ip)) = splited.next().map(|f| f.parse::<Ipv4Addr>()) {
                if let Some(Ok(port)) = splited.next().map(|f| f.parse::<u16>()) {
                    let proxy = Proxy {
                        ip,
                        port,
                        protocols: default_protocols.clone(),
                        ..Default::default()
                    };
                    if !self.send(proxy, tx, counter) {
                        break;
                    };
                }
            }
        }

        Ok(vec![])
    }
}
