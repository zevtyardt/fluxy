use std::{
    collections::HashSet,
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
    providers::{
        free_proxy_list::FreeProxyListProvider, github::GithubRepoProvider, IProxyTrait,
    },
};

pub struct ProxyFetcher {
    sender: mpsc::SyncSender<Option<Proxy>>,
    receiver: mpsc::Receiver<Option<Proxy>>,
    counter: Arc<AtomicUsize>,
    timer: time::Instant,
    providers: Vec<Arc<dyn IProxyTrait + Send + Sync>>,

    enforce_unique_ip: bool,
    unique_ip: HashSet<(Ipv4Addr, u16)>,
}

impl Default for ProxyFetcher {
    fn default() -> Self {
        let (sender, receiver) = mpsc::sync_channel(1024);
        Self {
            sender,
            receiver,
            counter: Arc::new(AtomicUsize::new(0)),
            timer: time::Instant::now(),
            providers: vec![],
            enforce_unique_ip: true,
            unique_ip: HashSet::new(),
        }
    }
}

impl ProxyFetcher {
    pub fn use_default_providers(&mut self) {
        self.providers = vec![
            Arc::new(FreeProxyListProvider::default()),
            Arc::new(GithubRepoProvider),
        ];
    }

    /// Ensure each proxy has unique IP, this will affect performance (default: true)
    pub fn enforce_unique_ip(&mut self, value: bool) {
        self.enforce_unique_ip = value;
    }

    #[allow(unused_must_use)]
    pub async fn gather(&self) -> anyhow::Result<JoinHandle<()>> {
        let providers = self.providers.clone();
        let mut tasks = vec![];
        for provider in providers.iter() {
            for source in provider.sources() {
                tasks.push((Arc::new(source), Arc::clone(provider)));
            }
        }

        #[cfg(feature = "log")]
        log::debug!("Searching proxies from {} available sources", tasks.len(),);

        let ua = UserAgent().fake::<&str>();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .user_agent(ua)
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()?;
        let counter = self.counter.clone();
        counter.store(0, std::sync::atomic::Ordering::Relaxed);
        let sender = self.sender.clone();

        let handle = tokio::spawn(async move {
            let pool = Pool::bounded(10);
            for (source, provider) in tasks.iter() {
                let source = Arc::clone(source);
                let provider = Arc::clone(provider);

                let tx = sender.clone();
                let client = client.clone();
                let counter = Arc::clone(&counter);
                pool.spawn(async move {
                    if let Ok(html) = provider.fetch(&client, source.url.as_ref()).await {
                        let protocols = source.default_protocols.clone();
                        provider.scrape(html, &tx, &counter, protocols).await;
                    }
                })
                .await;
            }

            while pool.busy_permits().unwrap_or(0) != 0 {
                time::sleep(Duration::from_millis(50)).await;
            }
            sender.send(None).unwrap_or_default();
        });
        Ok(handle)
    }
}

impl Drop for ProxyFetcher {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        let total_proxies = self.counter.load(std::sync::atomic::Ordering::Acquire);

        #[cfg(feature = "log")]
        log::debug!(
            "Proxy gather completed in {:?}. {} proxies where found. ",
            self.timer.elapsed(),
            total_proxies
        );
        self.sender.send(None).unwrap_or_default();
    }
}

impl Iterator for ProxyFetcher {
    type Item = Proxy;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(proxy) = self.receiver.recv().unwrap_or_default() {
            if self.enforce_unique_ip {
                if !self.unique_ip.contains(&(proxy.ip, proxy.port)) {
                    self.unique_ip.insert((proxy.ip, proxy.port));
                    return Some(proxy);
                } else {
                    self.counter
                        .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                    return self.next();
                }
            }
            return Some(proxy);
        }
        None
    }
}
