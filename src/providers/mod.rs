use std::sync::{atomic::AtomicUsize, mpsc, Arc};

use async_trait::async_trait;
use reqwest::{Client, Url};
use scraper::Html;

use crate::models::{Anonymity, Protocol, Proxy, Source};

pub mod free_proxy_list;

#[async_trait]
pub trait IProxyTrait {
    fn sources(&self) -> Vec<Source>;

    async fn fetch(&self, client: &Client, url: &str) -> anyhow::Result<Html>;

    async fn scrape(
        &self, html: Html, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>, default_protocols: Vec<Arc<Protocol>>,
    ) -> anyhow::Result<Vec<Source>>;

    fn send(
        &self, proxy: Proxy, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>,
    ) -> bool {
        if tx.send(Some(proxy)).is_ok() {
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return true;
        }
        false
    }
}
