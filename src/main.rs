use fluxy::{fetcher::ProxyFetcher, setup_log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "log")]
    setup_log(log::LevelFilter::Trace)?;

    let mut f = ProxyFetcher::default();
    let handle = f.spawn().await;
    #[cfg(feature = "log")]
    log::info!("{:?}", f.next());
    handle.await?;
    Ok(())
}
