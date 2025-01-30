use styledlog::*;

/// Tests for the `hex_to_rgba` function in the colorful module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests conversion of a 6-digit hex color to RGBA.
    #[test]
    fn test_convert_hex_color_to_rgba() {
        let hex_color = "#054eee";
        assert_eq!(hex_to_rgba(&hex_color), Ok((5, 78, 238, 1.0)));
    }

    /// Tests conversion of a 3-digit shorthand hex black color to RGBA.
    #[test]
    fn test_convert_hex_black_color_3_to_rgba() {
        let hex_color = "#000";
        assert_eq!(hex_to_rgba(&hex_color), Ok((0, 0, 0, 1.0)));
    }

    /// Tests conversion of a 6-digit hex black color to RGBA.
    #[test]
    fn test_convert_hex_black_color_6_to_rgba() {
        let hex_color = "#000000";
        assert_eq!(hex_to_rgba(&hex_color), Ok((0, 0, 0, 1.0)));
    }

    /// Tests conversion of a 6-digit hex white color to RGBA.
    #[test]
    fn test_convert_hex_white_color_to_rgba() {
        let hex_color = "#ffffff";
        assert_eq!(hex_to_rgba(&hex_color), Ok((255, 255, 255, 1.0)));
    }

    /// Tests conversion of a hex color with alpha to RGBA.
    #[test]
    fn test_hex_color_with_alpha() {
        assert_eq!(hex_to_rgba(&"#054eee80"), Ok((5, 78, 238, 0.5019608)));
    }
}
