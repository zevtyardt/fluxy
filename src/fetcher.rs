use std::{
    collections::HashSet,
    net::Ipv4Addr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use tokio::{task::JoinHandle, time};
use tokio_task_pool::Pool;

use crate::{
    geoip::GeoIp,
    models::{Proxy, ProxyFetcherConfig, Source, Type},
    providers::{FreeProxyListProvider, GithubRepoProvider, IProxyTrait, ProxyscrapeProvider},
};

/// Responsible for fetching proxies from various sources.
pub struct ProxyFetcher {
    sender: crossbeam_channel::Sender<Option<Proxy>>, // Channel sender for passing proxies.
    receiver: crossbeam_channel::Receiver<Option<Proxy>>, // Channel receiver for receiving proxies.
    counter: Arc<AtomicUsize>, // Counter for tracking the number of fetched proxies.
    timer: time::Instant,      // Timer for measuring elapsed time.
    elapsed: Option<Duration>, // Duration of the fetcher operation.
    geoip: Option<GeoIp>,      // Optional GeoIP instance for location lookups.
    providers: Vec<Arc<dyn IProxyTrait + Send + Sync>>, // List of proxy providers.
    unique_ip: HashSet<(Ipv4Addr, u16)>, // Set to track unique IPs.
    handler: Option<JoinHandle<()>>, // Handle for the fetching task.
    config: ProxyFetcherConfig, // Configuration for the proxy fetcher.
}

impl ProxyFetcher {
    /// Starts a new `ProxyFetcher` with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config`: The configuration for the proxy fetcher.
    ///
    /// # Returns
    ///
    /// A result containing the initialized `ProxyFetcher`.
    pub async fn gather(config: ProxyFetcherConfig) -> anyhow::Result<Self> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        //        let (sender, receiver) = mpsc::channel();
        let geoip = if config.enable_geo_lookup {
            Some(GeoIp::new().await?)
        } else {
            None
        };
        let providers: Vec<Arc<dyn IProxyTrait + Send + Sync>> = vec![
            Arc::new(FreeProxyListProvider::default()),
            Arc::new(GithubRepoProvider),
            Arc::new(ProxyscrapeProvider),
        ];

        let mut fles = Self {
            sender,
            receiver,
            counter: Arc::new(AtomicUsize::new(0)),
            timer: time::Instant::now(),
            elapsed: None,
            geoip,
            handler: None,
            providers,
            unique_ip: HashSet::new(),
            config,
        };
        fles.start().await?;
        Ok(fles)
    }
}

/// Executes the work of fetching proxies from a given provider.
async fn do_work(
    provider: Arc<dyn IProxyTrait + Send + Sync>,
    client: Arc<Client<HttpsConnector<HttpConnector>, Empty<Bytes>>>,
    source: Arc<Source>,
    tx: crossbeam_channel::Sender<Option<Proxy>>,
    counter: Arc<AtomicUsize>,
) -> anyhow::Result<()> {
    let html = provider
        .fetch(client, &source.url.to_string(), source.timeout)
        .await?;
    let types = source
        .default_types
        .clone()
        .into_iter()
        .map(|protocol| Type {
            protocol,
            checked: false,
        })
        .collect();
    provider.scrape(html, tx, counter, types).await
}

impl ProxyFetcher {
    /// Adds a custom proxy provider to the fetcher.
    ///
    /// # Arguments
    ///
    /// * `provider`: The provider to add.
    pub fn add_provider(&mut self, provider: Arc<dyn IProxyTrait + Send + Sync>) {
        self.providers.push(provider);
    }

    /// Gathers proxies from the configured providers.
    #[allow(unused_must_use)]
    async fn start(&mut self) -> anyhow::Result<()> {
        // Abort any ongoing gathering process if it exists.
        if let Some(handler) = &self.handler {
            handler.abort();
            self.handler = None;
        }

        let providers = self.providers.clone();
        let mut tasks = vec![];
        for provider in providers.iter() {
            for source in provider.sources() {
                tasks.push((Arc::new(source), Arc::clone(provider)));
            }
        }

        #[cfg(feature = "log")]
        log::debug!(
            "Proxy gather started. Collecting proxies from {} sources",
            tasks.len(),
        );

        let counter = self.counter.clone();
        counter.store(0, std::sync::atomic::Ordering::Relaxed);
        let sender = self.sender.clone();

        let client = Arc::new(
            Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(HttpsConnector::new()),
        );
        let concurrency_limit = self.config.concurrency_limit;
        let handler = tokio::spawn(async move {
            let pool = Pool::bounded(concurrency_limit);
            for (source, provider) in tasks.iter() {
                let source = Arc::clone(source);
                let provider = Arc::clone(provider);
                let client = Arc::clone(&client);
                let tx = sender.clone();
                let counter = Arc::clone(&counter);
                pool.spawn(async move {
                    let url = source.url.to_string();
                    if let Err(e) = do_work(provider, client, source, tx, counter).await {
                        #[cfg(feature = "log")]
                        log::error!("{}: {}", url, e);
                    }
                })
                .await;
            }
            // Wait for all tasks in the pool to complete.
            while pool.busy_permits().unwrap_or(0) != 0 {
                time::sleep(Duration::from_millis(50)).await;
            }
            sender.send(None).unwrap_or_default();
        });
        self.handler = Some(handler);
        Ok(())
    }

    /// Retrieves one proxy from the receiver.
    ///
    /// If geo lookup is enabled, it will apply geographic filtering.
    ///
    /// # Returns
    ///
    /// An optional `Proxy` if one is available, otherwise `None`.
    pub fn get_one(&mut self) -> Option<Proxy> {
        while let Some(mut proxy) = self.receiver.recv().ok()? {
            if let Some(geoip) = &self.geoip {
                proxy.geo = geoip.lookup(&proxy.ip);

                if !self.config.countries.is_empty()
                    && !proxy
                        .geo
                        .iso_code
                        .clone()
                        .map(|f| self.config.countries.contains(&f))
                        .unwrap_or(false)
                {
                    self.counter.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
            }

            if self.config.enforce_unique_ip {
                if self.unique_ip.insert((proxy.ip, proxy.port)) {
                    return Some(proxy);
                } else {
                    self.counter.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
            } else {
                return Some(proxy);
            }
        }
        None
    }
}

impl Iterator for ProxyFetcher {
    type Item = Proxy;

    /// Retrieves the next proxy from the fetcher.
    ///
    /// # Returns
    ///
    /// An optional `Proxy` if available, otherwise `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(proxy) = self.get_one() {
            return Some(proxy);
        }
        self.elapsed = Some(self.timer.elapsed());
        None
    }
}

impl Drop for ProxyFetcher {
    /// Cleans up resources when `ProxyFetcher` is dropped.
    fn drop(&mut self) {
        self.sender.send(None).unwrap_or_default();
        if let Some(handler) = &self.handler {
            handler.abort();
        }

        #[cfg(feature = "log")]
        let total_proxies = self.counter.load(std::sync::atomic::Ordering::Acquire);
        #[cfg(feature = "log")]
        log::debug!(
            "Proxy gather completed in {:?}. {} proxies were found",
            self.elapsed.unwrap_or(self.timer.elapsed()),
            total_proxies
        );
    }
}
