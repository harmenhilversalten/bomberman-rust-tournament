//! Helper utilities.

/// Normalize a name by trimming whitespace and converting to lowercase.
pub fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}
