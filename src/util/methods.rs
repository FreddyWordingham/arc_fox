//! Method macros.

/// Succinctly create a reference method for the given variable.
#[macro_export]
macro_rules! access {
    ($field:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> &$type {
            &self.$field
        }
    };

    ($field:ident, $setter:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> &$type {
            &self.$field
        }

        #[inline]
        #[must_use]
        pub fn $setter(&mut self) -> &mut $type {
            &mut self.$field
        }
    };
}

/// Succinctly define a struct and a simple constructor method.
#[macro_export]
macro_rules! new {
    ($name:ident { $( $field:ident: $type:ty ),* }) => {
        pub struct $name {
            $($field: $type,)*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $( $field, )*
                }
            }
        }
    };

}
