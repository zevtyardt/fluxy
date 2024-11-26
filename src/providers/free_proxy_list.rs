use std::{
    default,
    net::Ipv4Addr,
    sync::{atomic::AtomicUsize, mpsc, Arc},
};

use async_trait::async_trait;
use fake::{faker::internet::en::IPv4, Fake};
use reqwest::{Client, Url};
use scraper::{Html, Selector};

use super::IProxyTrait;
use crate::models::{Anonymity, Protocol, Proxy, Source};

pub struct FreeProxyListProvider {
    table: Selector,
    row: Selector,
    column: Selector,
}

impl Default for FreeProxyListProvider {
    fn default() -> Self {
        Self {
            table: Selector::parse("table > tbody").unwrap(),
            row: Selector::parse("tr").unwrap(),
            column: Selector::parse("td").unwrap(),
        }
    }
}

#[async_trait]
impl IProxyTrait for FreeProxyListProvider {
    fn sources(&self) -> Vec<Source> {
        vec![
            Source::new(
                "https://www.sslproxies.org/",
                vec![
                    Protocol::Http(Anonymity::Unknown),
                    Protocol::Https,
                    Protocol::Connect(80),
                    Protocol::Connect(25),
                ],
            ),
            Source::new(
                "https://free-proxy-list.net/uk-proxy.html",
                vec![
                    Protocol::Http(Anonymity::Unknown),
                    Protocol::Https,
                    Protocol::Connect(80),
                    Protocol::Connect(25),
                ],
            ),
            Source::new(
                "https://www.us-proxy.org/",
                vec![
                    Protocol::Http(Anonymity::Unknown),
                    Protocol::Https,
                    Protocol::Connect(80),
                    Protocol::Connect(25),
                ],
            ),
            Source::new(
                "https://free-proxy-list.net/",
                vec![
                    Protocol::Http(Anonymity::Unknown),
                    Protocol::Https,
                    Protocol::Connect(80),
                    Protocol::Connect(25),
                ],
            ),
            Source::new(
                "https://socks-proxy.net/",
                vec![Protocol::Socks4, Protocol::Socks5],
            ),
        ]
    }

    async fn fetch(&self, client: &Client, url: &str) -> anyhow::Result<Html> {
        let response = client.get(url).send().await?;
        let text = response.text().await?;
        Ok(Html::parse_document(&text))
    }

    async fn scrape(
        &self, html: Html, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>, default_protocols: Vec<Arc<Protocol>>,
    ) -> anyhow::Result<Vec<Source>> {
        if let Some(table) = html.select(&self.table).next() {
            for row in table.select(&self.row) {
                let mut col = row.select(&self.column).map(|i| i.inner_html());
                if let Some(Ok(ip)) = col.next().map(|f| f.parse::<Ipv4Addr>()) {
                    if let Some(Ok(port)) = col.next().map(|f| f.parse::<u16>()) {
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
        }
        Ok(vec![])
    }
}
