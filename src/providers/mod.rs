use std::sync::{atomic::AtomicUsize, mpsc, Arc};

use async_trait::async_trait;
use reqwest::{Client, Url};
use scraper::Html;

use crate::models::{Anonymity, Proxy, Source};

pub mod free_proxy_list;

#[async_trait]
pub trait IProxyTrait {
    fn sources(&self) -> Vec<Source>;

    async fn fetch(&self, client: &Client, url: Url) -> anyhow::Result<Html>;

    async fn scrape(
        &self, html: Html, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>,
    ) -> anyhow::Result<Vec<Source>>;

    fn send(
        &self, proxy: Proxy, tx: &mpsc::SyncSender<Option<Proxy>>,
        counter: &Arc<AtomicUsize>,
    ) {
        if let Err(e) = tx.send(Some(proxy)) {
            #[cfg(feature = "log")]
            log::error!("Failed to send proxy, reason: {}", e);
        } else {
            counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}
