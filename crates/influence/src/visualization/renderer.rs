//! Rendering utilities for influence maps.

use crate::core::{InfluenceError, InfluenceMap, InfluenceType};

/// Renders the selected influence layer as an ASCII grid.
/// Each value is formatted with two decimal places and separated by spaces.
/// A newline terminates each row.
pub fn render_ascii(map: &InfluenceMap, layer: InfluenceType) -> Result<String, InfluenceError> {
    let mut out = String::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let value = match layer {
                InfluenceType::Danger => map.danger_at(x, y)?,
                InfluenceType::Opportunity => map.opportunity_at(x, y)?,
            };
            out.push_str(&format!("{value:.2} "));
        }
        if map.width() > 0 {
            out.pop();
        }
        out.push('\n');
    }
    Ok(out)
}
