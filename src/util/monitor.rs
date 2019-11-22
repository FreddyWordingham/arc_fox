//! Monitor structure.

use super::progress::bar;
use contracts::pre;
use indicatif::ProgressBar;
use std::sync::Mutex;

/// Monitor structure implementation.
/// Helps run multi-threaded portions of code.
pub struct Monitor {
    /// Total increments to read.
    total: u64,
    /// Counts from each thread.
    counts: Mutex<Vec<u64>>,
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
            counts: Mutex::new(vec![0; num_threads]),
            pb: bar(msg, total),
        }
    }
}
