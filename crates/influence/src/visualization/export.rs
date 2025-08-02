//! Export utilities for influence maps.

use crate::core::{InfluenceError, InfluenceMap, InfluenceType};

/// Exports the selected influence layer to a CSV string.
/// Values are formatted with two decimal places.
pub fn export_csv(map: &InfluenceMap, layer: InfluenceType) -> Result<String, InfluenceError> {
    let mut out = String::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let value = match layer {
                InfluenceType::Danger => map.danger_at(x, y)?,
                InfluenceType::Opportunity => map.opportunity_at(x, y)?,
            };
            out.push_str(&format!("{value:.2}"));
            if x + 1 < map.width() {
                out.push(',');
            }
        }
        if y + 1 < map.height() {
            out.push('\n');
        }
    }
    Ok(out)
}
