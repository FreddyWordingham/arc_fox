//! Progress bar implementation.

use indicatif::{ProgressBar, ProgressStyle};

pub fn bar(title: &str, size: u64) -> ProgressBar {
    let bar = ProgressBar::new(size);
    bar.set_message(title);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}",
            )
            .progress_chars("\\/"),
    );

    bar
}
