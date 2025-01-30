use colored::{Color, ColoredString, Colorize};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SHOWN_LEVELS: Arc<Mutex<Vec<&'static str>>> = {
        let levels = Vec::new(); // Start with an empty Vec
        Arc::new(Mutex::new(levels))
    };
}

#[derive(Copy, Clone)]
pub struct Style {
    pub color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Style {
    pub fn new() -> Self {
        Self {
            color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn apply(&self, formatted_message: ColoredString) -> ColoredString {
        let mut styled = formatted_message;

        if let Some(color) = self.color {
            styled = styled.color(color);
        }
        if self.bold {
            styled = styled.bold();
        }
        if self.italic {
            styled = styled.italic();
        }
        if self.underline {
            styled = styled.underline();
        }

        styled
    }
}

// Implement the Default trait
impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Logs a message based on the provided level, style, and action.
///
/// # Arguments
///
/// * `newline_before` - Optional string that is printed before the log level (can be empty or contain "\n").
/// * `level` - The log level (as a `&str`).
/// * `style` - The style of the log level, including color, boldness, and italics.
/// * `message` - The message to log.
/// * `action` - The action to perform, can be "show" to print the message, or "hide" to suppress logging.
pub fn log_level(
    newline_before: &str,
    level: &str,
    style_level: Style,
    message: &str,
    action: &str,
) {
    // Lock access to the global variable
    let shown_levels = SHOWN_LEVELS.lock().unwrap();

    match action {
        "show" => {
            // Check if level is in the allowed list of shown levels
            if shown_levels.contains(&level) {
                let formatted_level: ColoredString = format!("[{}]", level.to_uppercase()).into();
                let styled_level = style_level.apply(formatted_level);

                // Print newline before the log level if requested
                print!("{}", newline_before);

                // Now print the log level and the message
                println!("{} {}", styled_level, message);
            }
        }
        "hide" => {
            // Do nothing, suppress the log
        }
        _ => {
            // No output for any other action
        }
    }
}

/// Adds a log level to the allowed list.
///
/// # Arguments
///
/// * `level` - The log level to be added (must have a static lifetime).
pub fn add_level(level: &'static str) {
    let mut shown_levels = SHOWN_LEVELS.lock().unwrap();
    if !shown_levels.contains(&level) {
        shown_levels.push(level);
    }
}

/// Removes a log level from the allowed list.
///
/// # Arguments
///
/// * `level` - The log level to be removed from the allowed list.
pub fn remove_level(level: &str) {
    let mut shown_levels = SHOWN_LEVELS.lock().unwrap();
    shown_levels.retain(|&x| x != level); // Remove specified level
}
