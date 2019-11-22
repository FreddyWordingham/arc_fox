//! Monitor structure.

use super::progress::bar;
use contracts::pre;
use indicatif::ProgressBar;

/// Monitor structure implementation.
/// Helps run multi-threaded portions of code.
pub struct Monitor {
    /// Total increments to read.
    total: u64,
    /// Counts from each thread.
    counts: Vec<u64>,
    /// Progress bar.
    pb: ProgressBar,
}

impl Monitor {
    /// Construct a new instance.
    #[pre(total > 0)]
    #[pre(num_threads > 0)]
    pub fn new(msg: &'static str, total: u64, num_threads: usize) -> Self {
        Self {
            total,
            counts: vec![0; num_threads],
            pb: bar(msg, total),
        }
    }

    /// Determine if another increment is possible.
    pub fn inc(&mut self, thread_id: usize) -> bool {
        let sum: u64 = self.counts.iter().sum();
        if sum < self.total {
            self.pb.inc(1);
            self.counts[thread_id] += 1;
            return true;
        }

        false
    }

    /// Finish with a message.
    pub fn finish_with_message(&mut self, msg: &'static str) {
        self.pb.finish_with_message(msg);
    }
}
