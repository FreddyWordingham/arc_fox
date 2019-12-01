//! Serial-Progress bar structure.

use contracts::pre;
use indicatif::{ProgressBar, ProgressStyle};

/// Serial-Progress bar structure implementation.
/// Helps run single-threaded portions of code.
pub struct SerialBar {
    /// Total increments to read.
    total: u64,
    /// Counts.
    counts: u64,
    /// Progress bar.
    pb: ProgressBar,
}

impl SerialBar {
    /// Construct a new instance.
    #[pre(total > 0)]
    pub fn new(msg: &'static str, total: u64) -> Self {
        let pb = ProgressBar::new(total);
        pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.cyan} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}",
                    )
                    .progress_chars("\\/"),
            );
        pb.set_message(msg);

        Self {
            total,
            counts: 0,
            pb,
        }
    }

    /// Determine if another increment is possible.
    pub fn inc(&mut self) -> Option<u64> {
        if self.counts < self.total {
            self.pb.inc(1);
            self.counts += 1;
            return Some(self.counts);
        }

        None
    }

    /// Finish with a message.
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
