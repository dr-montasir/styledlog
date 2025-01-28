use styledlog::*;

fn main() {
    println!();

    let formatted_text = style_text!(
        "{{here}} {{is}} \\\"the\" {{message1}}: This {{is}} a {{test}} {{message2}}.",
        here => "Here".to_custom_color("#054eee80"),
        is => "is".to_custom_color("#ffff21"),
        message1 => "message".blue(),
        message2 => "message".green().bold().italic().underline(),
        test => "test".to_custom_color("#e1e")
    );

    println!("{}", formatted_text);

    println!();

    let styledtitle = style_text!("{{T}}{{abl}}{{e}}", T => "T".cyan().italic(), abl => "abl".normal(), e => "e".cyan()).bold().dimmed();
    println!("{}", styledtitle);

    // Define header with ColoredString
    let header: Metadata = &[
        "Name".cyan(),
        "Age".magenta(),
        "Favorite Color"
            .to_uppercase()
            .to_custom_color("#4f1")
            .bold(),
        "🔥".into(),
    ];

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

    println!(); // Adding a new line for better separation

    // Call the function without a header and footer, using empty slices instead.
    print_table(&[], &users, &[], 9, "*".blue(), None);

    println!();

    // Call the function without a header and footer, using empty slices instead (custom repeat 35).
    print_table(&[], &users, &[], 9, "x".blue(), Some(35));

    println!();

    // Call the function without a header and footer, using empty slices instead (sep = "").
    print_table(&[], &users, &[], 9, "".normal(), None);

    let color_with_alpha = "#054eee80"; // Test with alpha
    let rgba_with_alpha = hex_to_rgba(color_with_alpha);
    println!(
        "RGBA values for {}: {:?}",
        color_with_alpha, rgba_with_alpha
    );

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
