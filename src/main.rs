use fluxy::fetcher::ProxyFetcher;
#[cfg(feature = "log")]
use fluxy::setup_log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "log")]
    setup_log(log::LevelFilter::Trace)?;

    let mut f = ProxyFetcher::default();
    f.use_default_providers();
    let handle = f.gather().await?;

    #[cfg(feature = "log")]
    log::info!("{}", f.count());
    handle.await?;

    Ok(())
}
