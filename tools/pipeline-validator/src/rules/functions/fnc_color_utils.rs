// <FILE>tools/pipeline-validator/src/rules/functions/fnc_color_utils.rs</FILE> - <DESC>Color utility functions for rule evaluation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - color contrast validation</WCTX>
// <CLOG>Initial creation - color_distance, is_dark, luminance helpers</CLOG>

use serde_json::Value;

/// RGB color representation
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Calculate relative luminance (0.0 to 1.0)
    /// Using sRGB luminance formula: 0.2126*R + 0.7152*G + 0.0722*B
    pub fn luminance(&self) -> f64 {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Check if color is considered "dark" (luminance < 0.3)
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.3
    }

    /// Euclidean distance to another color in RGB space
    pub fn distance(&self, other: &Rgb) -> f64 {
        let dr = self.r as f64 - other.r as f64;
        let dg = self.g as f64 - other.g as f64;
        let db = self.b as f64 - other.b as f64;
        (dr * dr + dg * dg + db * db).sqrt()
    }
}

/// Parse a color from JSON value
/// Supports formats:
/// - {"type": "rgb", "r": 255, "g": 220, "b": 140}
/// - {"r": 255, "g": 220, "b": 140}
/// - Named colors like "White", "Black", etc.
pub fn parse_color(value: &Value) -> Result<Rgb, String> {
    match value {
        Value::Object(obj) => {
            // Check for RGB object
            let r = obj.get("r").and_then(|v| v.as_u64()).map(|v| v as u8);
            let g = obj.get("g").and_then(|v| v.as_u64()).map(|v| v as u8);
            let b = obj.get("b").and_then(|v| v.as_u64()).map(|v| v as u8);

            match (r, g, b) {
                (Some(r), Some(g), Some(b)) => Ok(Rgb { r, g, b }),
                _ => Err("Color object missing r, g, or b fields".to_string()),
            }
        }
        Value::String(s) => {
            // Named color support
            match s.to_lowercase().as_str() {
                "white" => Ok(Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                "black" => Ok(Rgb { r: 0, g: 0, b: 0 }),
                "red" => Ok(Rgb { r: 255, g: 0, b: 0 }),
                "green" => Ok(Rgb { r: 0, g: 255, b: 0 }),
                "blue" => Ok(Rgb { r: 0, g: 0, b: 255 }),
                "yellow" => Ok(Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                }),
                "cyan" => Ok(Rgb {
                    r: 0,
                    g: 255,
                    b: 255,
                }),
                "magenta" => Ok(Rgb {
                    r: 255,
                    g: 0,
                    b: 255,
                }),
                "gray" | "grey" => Ok(Rgb {
                    r: 128,
                    g: 128,
                    b: 128,
                }),
                "lightyellow" => Ok(Rgb {
                    r: 255,
                    g: 255,
                    b: 224,
                }),
                _ => Err(format!("Unknown named color: {}", s)),
            }
        }
        _ => Err("Invalid color value".to_string()),
    }
}

/// Calculate Euclidean distance between two colors
pub fn color_distance(color1: &Value, color2: &Value) -> Result<f64, String> {
    let c1 = parse_color(color1)?;
    let c2 = parse_color(color2)?;
    Ok(c1.distance(&c2))
}

/// Check if a color is dark (low luminance)
pub fn is_dark(color: &Value) -> Result<bool, String> {
    let c = parse_color(color)?;
    Ok(c.is_dark())
}

/// Get luminance of a color (0.0 to 1.0)
pub fn luminance(color: &Value) -> Result<f64, String> {
    let c = parse_color(color)?;
    Ok(c.luminance())
}

/// Get maximum luminance of two colors
pub fn max_luminance(color1: &Value, color2: &Value) -> Result<f64, String> {
    let l1 = luminance(color1)?;
    let l2 = luminance(color2)?;
    Ok(l1.max(l2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_rgb_color() {
        let json = json!({"type": "rgb", "r": 255, "g": 128, "b": 64});
        let rgb = parse_color(&json).unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);
    }

    #[test]
    fn test_parse_named_color() {
        let json = json!("White");
        let rgb = parse_color(&json).unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn test_color_distance() {
        let white = json!({"r": 255, "g": 255, "b": 255});
        let black = json!({"r": 0, "g": 0, "b": 0});
        let dist = color_distance(&white, &black).unwrap();
        // sqrt(255^2 + 255^2 + 255^2) ≈ 441.67
        assert!((dist - 441.67).abs() < 1.0);
    }

    #[test]
    fn test_is_dark() {
        let dark = json!({"r": 25, "g": 20, "b": 10});
        let light = json!({"r": 255, "g": 255, "b": 255});
        assert!(is_dark(&dark).unwrap());
        assert!(!is_dark(&light).unwrap());
    }

    #[test]
    fn test_similar_colors_low_distance() {
        // The gold-on-gold problem from border_gold_luxury
        let base_fg = json!({"r": 255, "g": 220, "b": 140});
        let glisten_head = json!({"r": 255, "g": 220, "b": 120});
        let dist = color_distance(&base_fg, &glisten_head).unwrap();
        // Only 20 units apart - very similar!
        assert!(dist < 50.0, "Distance {} should be < 50", dist);
    }
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_color_utils.rs</FILE> - <DESC>Color utility functions for rule evaluation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
