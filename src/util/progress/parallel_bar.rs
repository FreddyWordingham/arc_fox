//! Parallel-Progress bar structure.

use contracts::pre;
use indicatif::{ProgressBar, ProgressStyle};

/// Parallel-Progress bar structure implementation.
/// Helps run multi-threaded portions of code.
pub struct ParallelBar {
    /// Total increments to read.
    total: u64,
    /// Counts from each thread.
    counts: Vec<u64>,
    /// Progress bar.
    pb: ProgressBar,
}

impl ParallelBar {
    /// Construct a new instance.
    #[pre(total > 0)]
    #[pre(num_threads > 0)]
    pub fn new(msg: &'static str, total: u64, num_threads: usize) -> Self {
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
            counts: vec![0; num_threads],
            pb,
        }
    }

    /// Determine if the given increment is possible.
    #[pre(thread_id < self.counts.len())]
    pub fn inc(&mut self, thread_id: usize, inc: u64) -> Option<(u64, u64)> {
        let sum: u64 = self.counts.iter().sum();
        let remaining = self.total - sum;

        if remaining == 0 {
            return None;
        }

        let allocation = (remaining / (2 * self.counts.len() as u64)).min(inc).max(1);
        self.pb.inc(allocation);
        self.counts[thread_id] += allocation;

        Some((sum, sum + allocation))
    }

    /// Finish with a message.
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
