//! Three-dimensional trait.

/// Types implementing this trait can be accessed in three-dimensions.
pub trait ThreeDimensional<T> {
    /// Access the X-dimension.
    fn x(&self) -> T;
    /// Access the Y-dimension.
    fn y(&self) -> T;
    /// Access the Z-dimension.
    fn z(&self) -> T;
}
