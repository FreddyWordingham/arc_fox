//! Progress bar implementation.

use indicatif::{ProgressBar, ProgressStyle};

/// Create a simple progress bar with nice formatting.
#[inline]
#[must_use]
pub fn bar(title: &str, size: u64) -> ProgressBar {
    let pb = ProgressBar::new(size);
    pb.set_message(title);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}",
            )
            .progress_chars("\\/"),
    );

    pb
}
