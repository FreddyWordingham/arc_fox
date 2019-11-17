//! Mapping functions.

use std::{collections::HashMap, fmt::Display};

/// Find the index of a key within a hashmap.
pub fn index_of_key<T: Display + PartialEq, S>(map: &HashMap<T, S>, key: &T) -> usize {
    for (i, (k, _v)) in map.iter().enumerate() {
        if key == k {
            return i;
        }
    }

    panic!("Unable to find key {} within the given map.", key);
}
