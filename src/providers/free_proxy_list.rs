use std::sync::mpsc;

use async_trait::async_trait;
use nipper::Document;
use reqwest::{Client, Url};

use super::IProxyTrait;
use crate::models::Proxy;

pub struct FreeProxyListProvider {}

#[async_trait]
impl IProxyTrait for FreeProxyListProvider {
    async fn fetch(&self, client: &Client) -> anyhow::Result<Document> {
        Ok(Document::default())
    }

    async fn scrape(
        &self, html: &Document, tx: mpsc::SyncSender<Proxy>,
    ) -> anyhow::Result<Option<Url>> {
        Ok(None)
    }
}
