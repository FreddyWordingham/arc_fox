//! Identity trait.

/// Types implementing this trait can be requested for id.
pub trait Identity {
    /// Reference the identity key.
    fn id(&self) -> &str;
}

/// Reference the first object with a matching id.
pub fn find_by_id<'a, T: Identity>(objs: &'a Vec<T>, id: &str) -> &'a T {
    for obj in objs {
        if obj.id() == id {
            return &obj;
        }
    }

    panic!("Could not find id: {}", id);
}
