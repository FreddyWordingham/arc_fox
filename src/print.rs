//! Printing module.

pub mod format;
pub mod term;

/// Report a value and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($expression: expr) => {
        log::info!("{: <31}: {}", stringify!($expression), $expression);
    };
    ($desc: tt, $expression: expr) => {
        log::info!("{: <31}: {}", $desc, $expression);
    };
    ($desc: tt, $expression: expr, $units: tt) => {
        log::info!("{: <31}: {} [{}]", $desc, $expression, $units);
    };
}
