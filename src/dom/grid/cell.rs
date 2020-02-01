//! Cell structure implementation.

use crate::{access, dom::Name, geom::Aabb, uni::State};

/// Cell holding local information.
pub struct Cell {
    /// Boundary.
    bound: Aabb,
    /// Central material.
    mat: Name,
    // /// Intersecting interface triangles.
    // inter_tris: Vec<(&'a (Name, Interface), Vec<&'a SmoothTriangle>)>,
    /// Local chemical state.
    state: State,
}

impl Cell {
    access!(bound, Aabb);
    access!(mat, Name);
    access!(state, state_mut, State);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(bound: Aabb, mat: Name, state: State) -> Self {
        Self { bound, mat, state }
    }
}
