//! Utility functions and structures.

/// Get the binary name.
pub fn bin_name() -> String {
    std::env::current_exe()
        .unwrap()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
