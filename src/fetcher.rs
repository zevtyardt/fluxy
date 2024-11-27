use std::{
    net::Ipv4Addr,
    sync::{atomic::AtomicUsize, mpsc, Arc},
    time::Duration,
};

use fake::{
    faker::internet::en::{IPv4, UserAgent},
    Fake,
};
use reqwest::ClientBuilder;
use tokio::{task::JoinHandle, time};
use tokio_task_pool::Pool;

use crate::{
    models::Proxy,
    providers::{free_proxy_list::FreeProxyListProvider, IProxyTrait},
};

pub struct ProxyFetcher {
    sender: mpsc::SyncSender<Option<Proxy>>,
    receiver: mpsc::Receiver<Option<Proxy>>,
    counter: Arc<AtomicUsize>,
    providers: Vec<Arc<dyn IProxyTrait + Send + Sync>>,
}

impl Default for ProxyFetcher {
    fn default() -> Self {
        let (sender, receiver) = mpsc::sync_channel(1024);
        Self {
            sender,
            receiver,
            counter: Arc::new(AtomicUsize::new(0)),
            providers: vec![],
        }
    }
}

impl ProxyFetcher {
    pub fn use_default_providers(&mut self) {
        self.providers = vec![Arc::new(FreeProxyListProvider::default())];
    }

    #[allow(unused_must_use)]
    pub async fn gather(&self) -> anyhow::Result<JoinHandle<()>> {
        #[cfg(feature = "log")]
        log::debug!("Proxy gather started.");

        let ua = UserAgent().fake::<&str>();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .user_agent(ua)
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()?;
        #[cfg(feature = "log")]
        log::debug!("Creating client with user-agent: {}", ua);

        let counter = self.counter.clone();
        counter.store(0, std::sync::atomic::Ordering::Relaxed);

        let providers = self.providers.clone();
        let sender = self.sender.clone();
        let handle = tokio::spawn(async move {
            let timer = time::Instant::now();
            let pool = Pool::bounded(5);
            for provider in providers.iter() {
                let provider = Arc::clone(provider);
                let client = client.clone();
                let tx = sender.clone();
                let counter = Arc::clone(&counter);
                pool.spawn(async move {
                    let mut sources = provider.sources();
                    while let Some(source) = sources.pop() {
                        if let Ok(html) = provider.fetch(&client, source.url).await {
                            if let Ok(next_sources) =
                                provider.scrape(html, &tx, &counter).await
                            {
                                sources.extend(next_sources);
                            }
                        }
                    }
                })
                .await;
            }

            while pool.busy_permits().unwrap_or(0) != 0 {
                time::sleep(Duration::from_millis(250)).await;
            }
            let total_proxies = counter.load(std::sync::atomic::Ordering::Acquire);
            #[cfg(feature = "log")]
            log::debug!(
                "Proxy gather completed in {:?}. {} proxies where found. ",
                timer.elapsed(),
                total_proxies
            );
            sender.send(None).unwrap_or_default();
        });
        Ok(handle)
    }
}

impl Iterator for ProxyFetcher {
    type Item = Proxy;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().unwrap_or_default()
    }
}
