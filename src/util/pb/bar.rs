//! Progress bar structure.

use indicatif::{ProgressBar, ProgressStyle};

/// Progress bar.
pub struct Bar {
    /// Total increments to read.
    total: u64,
    /// Counts from each thread.
    counts: Vec<u64>,
    /// Progress bar.
    pb: ProgressBar,
}

impl Bar {
    /// Construct a new instance.
    #[inline]
    pub fn new(msg: &'static str, total: u64, num_threads: usize) -> Self {
        let pb = ProgressBar::new(total);
        pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{bar:40.cyan/red}] [{pos}/{len}] {percent}% ({eta}) {msg}",
                    )
                    .progress_chars("{}}"),
            );
        pb.set_message(msg);

        Self {
            total,
            counts: vec![0; num_threads],
            pb,
        }
    }

    /// Determine if the given increment block is possible.
    #[inline]
    pub fn block(&mut self, thread_id: usize, inc: u64) -> Option<(u64, u64)> {
        let sum: u64 = self.counts.iter().sum();
        let remaining = self.total - sum;

        if remaining == 0 {
            return None;
        }

        let allocation = (remaining / self.counts.len() as u64).min(inc).max(1);
        self.counts[thread_id] += allocation;

        Some((sum, sum + allocation))
    }

    /// Updates the progress bar.
    #[inline]
    pub fn inc(&mut self) {
        self.pb.inc(1);
    }

    /// Finish with a message.
    #[inline]
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
