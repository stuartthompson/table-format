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
    use table::Table;
    use table::table_row::TableRow;
    use table::table_cell::TableCell;

    #[test]
    fn test_simple_vector_table() {
        let table = table!(
            "{c^:15:}" => "Food", "{c^:10:}" => "Count";
            "Fish", "3", "Pears", "5", "Pizza", "13"
        );

        let output = table.format();

        let expected = 
            match env::var("NO_COLOR") {
                Ok(_) => "+--------------------------+\n|     Food      |  Count   |\n+--------------------------+\n|Fish           |3         |\n+--------------------------+\n|Pears          |5         |\n+--------------------------+\n|Pizza          |13        |\n+--------------------------+\n",
                Err(_) => "+--------------------------+\n|\u{1b}[36m     Food      \u{1b}[0m|\u{1b}[36m  Count   \u{1b}[0m|\n+--------------------------+\n|Fish           |3         |\n+--------------------------+\n|Pears          |5         |\n+--------------------------+\n|Pizza          |13        |\n+--------------------------+\n",
            };

        assert_eq!(output, expected);
    }
}
