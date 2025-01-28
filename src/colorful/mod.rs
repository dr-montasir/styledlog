pub use colored::*;

// Convert hex color to RGBA and return a Result type for better error handling
pub fn hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, f32), String> {
    // Validate the length of the hex string
    match hex.len() {
        7 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[1..3], 16)
                .map_err(|_| "Invalid hex value for red".to_string())?;
            let g = u8::from_str_radix(&hex[3..5], 16)
                .map_err(|_| "Invalid hex value for green".to_string())?;
            let b = u8::from_str_radix(&hex[5..7], 16)
                .map_err(|_| "Invalid hex value for blue".to_string())?;
            Ok((r, g, b, 1.0)) // Fully opaque
        }
        9 => {
            // #RRGGBBAA format
            let r = u8::from_str_radix(&hex[1..3], 16)
                .map_err(|_| "Invalid hex value for red".to_string())?;
            let g = u8::from_str_radix(&hex[3..5], 16)
                .map_err(|_| "Invalid hex value for green".to_string())?;
            let b = u8::from_str_radix(&hex[5..7], 16)
                .map_err(|_| "Invalid hex value for blue".to_string())?;
            let a = u8::from_str_radix(&hex[7..9], 16)
                .map_err(|_| "Invalid hex value for alpha".to_string())?;
            let alpha = a as f32 / 255.0; // Scale from 0-255 to 0.0-1.0
            Ok((r, g, b, alpha)) // Return the parsed values
        }
        4 => {
            // #RGB format
            let r = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16)
                .map_err(|_| "Invalid hex value for red".to_string())?;
            let g = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16)
                .map_err(|_| "Invalid hex value for green".to_string())?;
            let b = u8::from_str_radix(&format!("{}{}", &hex[3..4], &hex[3..4]), 16)
                .map_err(|_| "Invalid hex value for blue".to_string())?;
            Ok((r, g, b, 1.0)) // Fully opaque
        }
        5 => {
            // #RGBA format
            let r = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16)
                .map_err(|_| "Invalid hex value for red".to_string())?;
            let g = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16)
                .map_err(|_| "Invalid hex value for green".to_string())?;
            let b = u8::from_str_radix(&format!("{}{}", &hex[3..4], &hex[3..4]), 16)
                .map_err(|_| "Invalid hex value for blue".to_string())?;
            let a = u8::from_str_radix(&format!("{}", &hex[4..5]), 16)
                .map_err(|_| "Invalid hex value for alpha".to_string())? as f32
                * (1.0 / 15.0); // Scale alpha from 0-F to 0.0-1.0
            Ok((r, g, b, a))
        }
        _ => Err("Unsupported hex format".to_string()), // Return an error message if the format is incorrect
    }
}

// Custom trait for coloring strings
pub trait CustomColorTrait {
    fn to_custom_color(self, hex: &str) -> ColoredString;
}

impl CustomColorTrait for &str {
    fn to_custom_color(self, hex: &str) -> ColoredString {
        // Handle the Result from hex_to_rgba
        match hex_to_rgba(hex) {
            Ok((r, g, b, a)) => {
                // Assume a white background
                let bg_r = 255;
                let bg_g = 255;
                let bg_b = 255;

                // Perform alpha blending with the assumed background color
                let r = ((r as f32 * a) + (bg_r as f32 * (1.0 - a))) as u8;
                let g = ((g as f32 * a) + (bg_g as f32 * (1.0 - a))) as u8;
                let b = ((b as f32 * a) + (bg_b as f32 * (1.0 - a))) as u8;

                // Apply the blended color
                self.truecolor(r, g, b)
            }
            Err(e) => {
                // Handle error (for instance, fallback to black if an error occurs)
                eprintln!("Error parsing hex value '{}': {}", hex, e);
                self.truecolor(0, 0, 0) // Return black
            }
        }
    }
}
