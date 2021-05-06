mod content;
mod content_iterator;
mod data_item;
mod data_source;
mod table_data_source;
pub mod table;

pub use content::ContentStyle;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use table::{Border, Table};
    use table::row::Row;
    use table::cell::Cell;
    use colored::Color;

    #[test]
    fn test_simple_table() {
        let table =
            table!(
                "{^:10:}" => "Food", "{^:10:}" => "Count";
                "Fish", "15", "Pizza", "10", "Steak", "6"
            );

        let output = table.format();
        println!("{}", output);

        let expected = "+----------+----------+\n|   Food   |  Count   |\n+----------+----------+\n|Fish      |15        |\n+----------+----------+\n|Pizza     |10        |\n+----------+----------+\n|Steak     |6         |\n+----------+----------+\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_simple_vector_table() {
        let table = table!(
            "{c^:15:}" => "Food", "{c^:10:}" => "Count";
            "Fish", "3", "Pears", "5", "Pizza", "13"
        );

        let output = table.format();
        println!("{}", output);

        let expected =
            match env::var("NO_COLOR") {
                Ok(_) => "+---------------+----------+\n|     Food      |  Count   |\n+---------------+----------+\n|Fish           |3         |\n+---------------+----------+\n|Pears          |5         |\n+---------------+----------+\n|Pizza          |13        |\n+---------------+----------+\n",
                Err(_) => "+---------------+----------+\n|\u{1b}[36m     Food      \u{1b}[0m|\u{1b}[36m  Count   \u{1b}[0m|\n+---------------+----------+\n|Fish           |3         |\n+---------------+----------+\n|Pears          |5         |\n+---------------+----------+\n|Pizza          |13        |\n+---------------+----------+\n",
            };

        assert_eq!(output, expected);
    }

    #[test]
    fn test_custom_border_table() {
        let mut table = table!(
            "{R^:10:}" => "Custom", "{C^:20:}" => "Borders";
            "are", "super fun", "and", "super awesome"
        );

        table.border = Border {
            top_left: '┌',
            top: '─',
            top_right: '┐',
            top_split: '┬',
            bottom_left: '└',
            bottom: '─',
            bottom_right: '┘',
            bottom_split: '┴',
            left: '│',
            left_split:'├',
            right: '│',
            right_split: '┤',
            vertical_split: '│',
            vertical_split_intersect_left: '┤',
            vertical_split_intersect_right: '├',
            vertical_split_intersect_both: '┼',
            horizontal_split: '─',
            horizontal_split_intersect_top: '┴',
            horizontal_split_intersect_bottom: '┬',
            horizontal_split_intersect_both: '┼',
            color: Color::Red
        };

        let output = table.format();
        println!("{}", output);

        let expected =
            match env::var("NO_COLOR") {
                Ok(_) => "┌──────────┬────────────────────┐\n│  Custom  │      Borders       │\n├──────────┼────────────────────┤\n│are       │super fun           │\n├──────────┼────────────────────┤\n│and       │super awesome       │\n└──────────┴────────────────────┘\n",
                Err(_) => "┌──────────┬────────────────────┐\n│\u{1b}[91m  Custom  \u{1b}[0m│\u{1b}[96m      Borders       \u{1b}[0m│\n├──────────┼────────────────────┤\n│are       │super fun           │\n├──────────┼────────────────────┤\n│and       │super awesome       │\n└──────────┴────────────────────┘\n",
            };

        assert_eq!(output, expected);
    }
}
