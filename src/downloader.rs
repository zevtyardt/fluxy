// Define the data model representing the status of your app.
// Make sure it is Send + Sync, so it can be read and written from different
// threads:

#[cfg(feature = "log")]
struct Progress(AtomicU64);

#[cfg(feature = "log")]
impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Downloading Geolite mmdb file: {}%",
            "INFO".bright_blue(),
            self.0.load(Ordering::Relaxed)
        )
    }
}
