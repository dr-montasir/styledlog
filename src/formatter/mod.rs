use super::colorful::*;

#[macro_export]
macro_rules! style_text {
    ($template:expr, $( $key:ident => $value:expr ),*) => {{
        let mut result : ColoredString = String::from($template).into();

        // Create a HashMap for replacements
        let mut replacements = std::collections::HashMap::new();

        // Fill the hashmap with key-value pairs
        $(
            replacements.insert(format!("{{{{{}}}}}", stringify!($key)), $value.to_string());
        )*

        // Replace the placeholders in the template with actual values
        for (key, value) in &replacements {
            result = result.replace(key, value).into();
        }

        result
    }};
}

// Type aliases for metadata and body
pub type Metadata<'a> = &'a [ColoredString];
pub type Rows = Vec<Vec<ColoredString>>;

pub fn print_table(
    header: &[ColoredString],
    rows: &Vec<Vec<ColoredString>>,
    footer: &[ColoredString],
    padding: usize,
    sep: ColoredString,
    rpt: Option<usize>,
) {
    let mut column_widths = Vec::new();

    // Determine the number of columns (based on the first row)
    let num_columns = if !rows.is_empty() {
        rows[0].len()
    } else {
        return; // Return if there are no rows
    };

    // Initialize column widths to 0
    column_widths.resize(num_columns, 0);

    // Track the maximum row length, considering the header and footer
    let mut max_length = 0;

    // Calculate maximum length for the header
    if !header.is_empty() {
        for (i, col) in header.iter().enumerate() {
            column_widths[i] = col.len();
        }
        max_length = column_widths.iter().map(|&width| width + padding).sum();
    }

    for row in rows {
        for (i, col) in row.iter().enumerate() {
            column_widths[i] = column_widths[i].max(col.len());
        }
        max_length = max_length.max(column_widths.iter().map(|&width| width + padding).sum());
    }

    // Calculate maximum length for the footer
    if !footer.is_empty() {
        for (i, col) in footer.iter().enumerate() {
            column_widths[i] = column_widths[i].max(col.len());
        }
        max_length = max_length.max(column_widths.iter().map(|&width| width + padding).sum());
    }

    // Check if sep is empty before proceeding
    let sep = if sep.len() == 0 {
        " ".normal() // Default to a single space with normal if sep is empty
    } else {
        sep
    };

    let separator = match rpt {
        Some(value) => sep.to_string().repeat(value),
        None => sep
            .to_string()
            .repeat(if (max_length - padding) % sep.len() == 0 {
                (max_length - padding) / sep.len()
            } else {
                ((max_length - padding) / sep.len()) + 1
            }),
    };

    // Print header if present
    if !header.is_empty() {
        println!("{}", separator);
        for (i, col) in header.iter().enumerate() {
            print!("{:<width$}", col, width = column_widths[i] + padding);
        }
        println!();
    }

    println!("{}", separator);
    for row in rows {
        for (i, col) in row.iter().enumerate() {
            print!("{:<width$}", col, width = column_widths[i] + padding);
        }
        println!();
    }
    println!("{}", separator);

    // Print footer if present
    if !footer.is_empty() {
        for (i, col) in footer.iter().enumerate() {
            print!("{:<width$}", col, width = column_widths[i] + padding);
        }
        println!();
        println!("{}", separator);
    }
}
