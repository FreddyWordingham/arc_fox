//! Common start-up operations.

use log::error;
use std::env::args;

/// Get the command line arguments.
pub fn get_args(hints: Vec<String>) -> Vec<String> {
    let args: Vec<String> = args().collect();

    if args.len() != (hints.len() + 1) {
        error!("Required call:\n{} {}", args[0], hints.join(" "));
        panic!("Invalid command line arguments!");
    }

    args
}
