//! Proto-structures which can be manifested into complete objects.

pub mod dir;
pub mod domain;
pub mod mat_map;
pub mod surf_map;
pub mod surface;

pub use self::dir::*;
pub use self::domain::*;
pub use self::mat_map::*;
pub use self::surf_map::*;
pub use self::surface::*;
