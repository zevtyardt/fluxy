use std::{net::Ipv4Addr, sync::mpsc, time::Duration};

use fake::{faker::internet::en::IPv4, Fake};
use tokio::task::JoinHandle;

use crate::models::Proxy;

pub struct ProxyFetcher {
    sender: mpsc::SyncSender<Proxy>,
    receiver: mpsc::Receiver<Proxy>,
}

impl ProxyFetcher {
    pub async fn gather(&self) -> JoinHandle<()> {
        #[cfg(feature = "log")]
        log::debug!("");

        tokio::spawn(async move {})
    }
}

impl Default for ProxyFetcher {
    fn default() -> Self {
        let (sender, receiver) = mpsc::sync_channel(512);
        Self { sender, receiver }
    }
}

impl Iterator for ProxyFetcher {
    type Item = Proxy;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(proxy) = self.receiver.recv_timeout(Duration::from_secs(5)) {
            Some(proxy)
        } else {
            None
        }
    }
}
