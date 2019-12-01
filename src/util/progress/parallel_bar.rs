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

    /// Determine if another increment is possible.
    #[pre(thread_id < self.counts.len())]
    pub fn inc(&mut self, thread_id: usize) -> Option<u64> {
        let sum: u64 = self.counts.iter().sum();
        if sum < self.total {
            // self.pb.inc(1);
            // println!("{:.2}%", 100.0 * sum as f64 / self.total as f64);
            self.counts[thread_id] += 1;
            return Some(sum);
        }

        None
    }

    /// Finish with a message.
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
