/// Converts a hex color string to an RGBA tuple.
///
/// # Arguments
///
/// * `hex` - A string slice that holds the hex color code. It can be:
///   - Standard 6-digit hex (e.g., "#RRGGBB")
///   - Shorthand 3-digit hex (e.g., "#RGB")
///   - Hex with alpha (e.g., "#RRGGBBAA" or "#RGBA")
///
/// # Returns
///
/// A `Result` containing the RGBA values if the conversion is successful, or an error message if the input is invalid.
///
/// The RGBA values are represented as follows:
///
/// * `red` - The red component of the color (0-255).
/// * `green` - The green component of the color (0-255).
/// * `blue` - The blue component of the color (0-255).
/// * `alpha` - The transparency level of the color (0.0-1.0).
///
/// # Errors
///
/// If the input string is not a valid hex color, this function returns an error message describing the problem.
pub mod hex_to_rgba;
