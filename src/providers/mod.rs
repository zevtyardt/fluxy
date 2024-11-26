use std::sync::mpsc;

use async_trait::async_trait;
use nipper::Document;
use reqwest::{Client, Url};

use crate::models::{Anonymity, Proxy};

pub mod free_proxy_list;

#[async_trait]
trait IProxyTrait {
    async fn fetch(&self, client: &Client) -> anyhow::Result<Document>;
    async fn scrape(
        &self, html: &Document, tx: mpsc::SyncSender<Proxy>,
    ) -> anyhow::Result<Option<Url>>;
}
