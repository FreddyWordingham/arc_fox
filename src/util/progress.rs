//! Progress functions.

use indicatif::{ProgressBar, ProgressStyle};

/// Construct a progress bar with a default style.
pub fn bar(msg: &'static str, ticks: u64) -> ProgressBar {
    let bar = ProgressBar::new(ticks);
    bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}",
                )
                .progress_chars("\\/"),
        );
    bar.set_message(msg);

    bar
}
