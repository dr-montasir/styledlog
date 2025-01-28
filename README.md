<div align="center">
  <br>
  <a href="https://crates.io/crates/styledlog">
    <img src="https://github.com/dr-montasir/styledlog/raw/HEAD/logo.svg" width="100">
  </a>
  <br><br>
  <span>
    <b>A Rust crate combining versatile logging features with styled output.</b>
  </span>
  <br><br>
  <a href="https://crates.io/crates/styledlog">
    <img src="https://github.com/dr-montasir/styledlog/raw/main/styledlog.gif" width="100%" height="auto">
  </a>
  <br><br>

[<img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20styledlog-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/dr-montasir/styledlog)[<img alt="crates.io" src="https://img.shields.io/crates/v/styledlog.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/styledlog)[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-styledlog-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/styledlog)[<img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">](https://choosealicense.com/licenses/apache-2.0)

  <h1>STYLEDLOG</h1>
  <p>
    The <b>styledlog</b> crate provides a set of utilities for logging messages with customizable styles including color, boldness, and italics, along with functionalities for styling tables and text based on user-defined templates.
  </p>
</div>

## Overview

This crate allows you to format log messages with colors and styles, convert hex colors to RGBA values, and print styled tables easily. It integrates well with the `colored` crate for rich text styling in the console.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Modules](#modules)
   - [colorful](#colorful)
   - [formatter](#formatter)
   - [level](#level)
4. [Examples](#examples)
   - [Coloring Text](#Coloring Text)
   - [Creating a Table with Style](#Creating a Table with Style)
   - [Logging Message Levels](#Logging Message Levels)
5. [License](#license)
6. [Contributing](#contributing)
7. [Author](#author)

## Installation

Run the following Cargo command in your project directory:

```shell
cargo add styledlog
```

or add styledlog to your Cargo.toml file:

```toml
[dependencies]
styledlog = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

## Usage

You can start using styledlog by including it in your main file:

```rust
use styledlog::*;
```

## Example

```rust
let formatted_text = style_text!(
    "{{here}} {{is}} \\\"the\" {{message1}}: This {{is}} a {{test}} {{message2}}.",
    here => "Here".to_custom_color("#054eee80"),
    is => "is".to_custom_color("#ffff21"),
    message1 => "message".blue(),
    message2 => "message".green().bold().italic().underline(),
    test => "test".to_custom_color("#e1e")
);

println!("{}", formatted_text);
```

To see the full features and usage examples of `styledlog`, please visit the following link:

[Full Features and Usage Examples](https://docs.rs/crate/styledlog/latest/source/examples/usage.rs)

## Modules

### colorful

This module provides color utilities for styling strings with colors defined in hex format, allowing for both general use and custom color definitions through the `CustomColorTrait`.

### formatter

This module contains functions for formatting output, including:

- `style_text!`: A _macro for_ <u>formatting</u> strings by replacing placeholders within a template. The _macro_ allows users to inject dynamic values into a string _while_ applying color and styling to those values. It creates an easy way to format complex output without the need _for_ <u>manual</u> string concatenation.

- `hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, f32), String>`: Converts a hex color string to RGBA values. It parses various hex color formats (e.g., `#RRGGBB`, `#RRGGBBAA`, `#RGB`, `#RGBA`) and handles errors gracefully.

- `print_table(...)`: A utility function _for_ <u>printing</u> a stylized table with headers, rows, and footers. It calculates column widths dynamically based on the content and formats the table _for_ <u>easy</u> readability _in_ the console.

### level

This module handles log levels and their styles. It contains:

- `log_level(newline_before: &str, level: &str, style_level: Style, message: &str, action: &str)`: Logs a message based on the provided log level and style. The action can be "show" or "hide".

- `add_level(level: &'static str)`: Adds a log level to the allowed list for display.

- `remove_level(level: &str)`: Removes a log level from the allowed list.

The `Style` struct allows you to define new styles for logs, including color bold, italic and underline embellishments.

## Examples

### Coloring Text

You can change the color of your text by calling:

```rust
let text = "Hello, World!".to_custom_color("#ff5733");
println!("{}", text);
```

### Creating a Table with Style

To print a table, use print_table:

```rust
use styledlog::*;

fn main() {
    println!(); // Adding a new line for better separation

    let styledtitle = style_text!("{{T}}{{abl}}{{e}}", T => "T".cyan().italic(), abl => "abl".normal(), e => "e\t<\t(1)\t>".cyan()).bold().dimmed();

    println!("{}\n", styledtitle);

    // Define header with ColoredString
    let header: Metadata = &[
        "Nmae".cyan(),
        "Age".magenta(),
        "Favorite Color"
            .to_uppercase()
            .to_custom_color("#4f1")
            .bold(),
        "🔥".into(),
    ];

    // Define rows (body)
    let users: Rows = vec![
        vec![
            "Ahmed Salih".normal(),
            "30".bold(),
            "Blue".blue(),
            "".normal(),
        ],
        vec![
            "Taiba".green(),
            "25".normal(),
            "Green".green(),
            "v".normal(),
        ],
        vec!["Umar".into(), "17".red(), "Red".red(), "".normal()],
    ];

    // Define footer with ColoredString
    let footer: Metadata = &[
        "End of Table".yellow().on_green(),
        "".normal(),
        " ".normal(),
        "🔥".normal().italic(),
    ];

    // Call the function with header and footer
    print_table(header, &users, footer, 2, "~o~".cyan(), None);

    println!();

    // Call the function without header and footer, using empty slices instead
    print_table(&[], &users, &[], 9, "*".blue(), None);

    println!();

    // Call the function without header and footer, using empty slices instead  (custom repeat 36)
    print_table(&[], &users, &[], 9, "x".blue(), Some(36));

    println!();

    // Call the function without header and footer, using empty slices instead
    print_table(&[], &users, &[], 9, "".normal(), None);
}
```

### Logging Message Levels

This is an example of how to log message levels:

```rust
use styledlog::*;

fn main() {
    // Add log levels that we want to show
    add_level("info");
    add_level("warn");
    add_level("error");
    add_level("debug");
    add_level("error/handler");
    add_level("debug/handler");

    // Define styles for different levels
    let info_level_style = Style::new().color(Color::Green).bold();
    let error_level_style = Style::new().color(Color::Red).bold();
    let warn_level_style = Style::new().color(Color::BrightYellow).bold();
    let debug_level_style = Style::new()
        .color(Color::Yellow)
        .bold()
        .italic()
        .underline();

    // Log messages
    log_level(
        "\n",
        "info",
        info_level_style,
        "This is an info message",
        "show",
    );
    log_level(
        "\n",
        "warn",
        warn_level_style,
        "\nThis is a warning message",
        "show",
    );

    let style_part_of_message =
        style_text!("{{this}} {{is}} an", this=>"This".to_custom_color("#0de"), is=>"is".magenta());
    log_level(
        "\n",
        "error",
        error_level_style,
        &format!(
            "{} {} {}",
            style_part_of_message,
            "error".red(),
            "message".on_truecolor(135, 28, 167)
        ),
        "show",
    );

    // Attempt to log a message with a level that's not displayed
    log_level(
        "\n",
        "debug",
        debug_level_style,
        "This is a debug message",
        "show",
    ); // Should not print anything

    let message = format!(
        "{}",
        "This is a custom error handler message\n".to_custom_color("#1e1707")
    );
    log_level("\n", "error/handler", Style::new(), &message, "show");

    // Attempt to log a message with a level that's not displayed
    log_level(
        "\n",
        "warn/handler",
        warn_level_style,
        "This is a custom warning handler message\n",
        "show",
    ); // Should not print anything

    log_level(
        "\n",
        "debug/handler",
        warn_level_style,
        &format!(
            "{}",
            "This is a custom debug handler message\n"
                .truecolor(0, 255, 136)
                .underline()
        ),
        "hide",
    ); // action: hide => Should not print anything
}
```

## License

This project is licensed under the MIT or Apache 2.0 License - see the LICENSE file for details.

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to submit an issue or a pull request.

---

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
