//! Utility module.

pub mod exec;
pub mod monitor;
pub mod progress;
pub mod range;
pub mod tag;

pub use self::exec::*;
pub use self::monitor::*;
pub use self::range::*;
pub use self::tag::*;

/// Concisely construct a hashmap.
#[macro_export]
macro_rules! map {
    ($( $key:expr => $val:expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    }
}
