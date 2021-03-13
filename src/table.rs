mod border;
pub mod column_break;
pub mod table_row;
pub mod table_cell;

use std::str::FromStr;
use border::Border;
pub use column_break::{ColumnBreak};
use super::data_item::DataItem;
use table_cell::TableCell;
use table_row::TableRow;
use crate::vec_data_source::VecDataSource;
use crate::row;

#[allow(unused_macros)]
#[macro_export]
macro_rules! table {
    ( $($style:literal=>$header:literal),*; $($data:literal),* ) => {
        Table::from_vec(
            // Header specification
            row!($($style => $header),*),
            // Data
            vec!($($data),*)
        )
    };
    // ($breaks:expr, $headers:expr, $($content:expr),*) => {{
    //     let mut data = Vec::new();
    //     $( data.push($content); )*
    //     Table::from_vec(
    //         $breaks,
    //         $headers,
    //         data
    //     )
    // }};
}

#[derive(Debug)]
pub struct Table {
    border: Border,
    column_breaks: Vec<ColumnBreak>,
    column_headers: TableRow,
    row_headers: Vec<TableCell>,
    data_rows: Vec<TableRow>
}

impl Table {
    // Returns an empty table
    pub fn empty() -> Table {
        Table {
            border: Border::default(),
            column_breaks: Vec::new(),
            column_headers: TableRow::new(),
            row_headers: Vec::new(),
            data_rows: Vec::new(),
        }
    }

    /// Returns a table from the supplied parameters.
    /// 
    /// # Arguments
    /// 
    /// * `border` - Describes the table border.
    /// * `column_breaks` - Column breaks describe header row widths.
    /// * `column_headers` - The content for the column headers.
    /// * `row_headers` - The content for the row headers.
    /// * `data_rows` - The rows in the table body.
    pub fn new(
        border: Border,
        column_breaks: Vec<ColumnBreak>,
        column_headers: TableRow,
        row_headers: Vec<TableCell>,
        data_rows: Vec<TableRow>,
    ) -> Table {
        Table {
            border,
            column_breaks,
            column_headers,
            row_headers,
            data_rows
        }
    }

    /// Returns a table built from a string vector data source.
    /// 
    /// # Arguments
    /// 
    /// * `column_headers` - The header row describes how to split the data.
    /// * `data` - A vector containing the data for the table body.
    pub fn from_vec(
        column_headers: TableRow,
        data: Vec<&str>
    ) -> Table {
        // Build data items from string vector source
        let d: Vec<DataItem> = 
            data.iter().map(|i| DataItem::from_str(i).unwrap())
                .collect::<Vec<DataItem>>();

        Table::from_data_source(
            column_headers,
            Vec::new(),
            d.iter()
        )
    }

    /// Returns a table built from a data source.
    /// 
    /// # Arguments
    /// 
    /// * `column_headers` - The header row describes how to split the data.
    /// * `row_headers` - The row headers to put before each row.
    /// * `data_source` - An iterable source providing the table body data.
    pub fn from_data_source<'a, I>(
        column_headers: TableRow,
        row_headers: Vec<TableCell>,
        data_source: I,
    ) -> Table 
        where 
            I: Iterator<Item=&'a DataItem>
    {
        let mut data_rows = Vec::new();
        
        // Derive column breaks from column headers
        let mut column_breaks: Vec<ColumnBreak> = Vec::new();
        for cell in column_headers.iter() {
            column_breaks.push(cell.get_break_from_content());
        }
        
        println!("Column Breaks: {:?}", column_breaks);

        // Create a new row
        let mut row_ix = 0;
        data_rows.push(TableRow::new());

        let mut break_ix = 0;

        for item in data_source {
            // Add a new row if needed
            if break_ix == column_breaks.len() {
                break_ix = 0;
                data_rows.push(TableRow::new());
                row_ix += 1;
            }

            data_rows[row_ix].add_cell(
                TableCell::from_data_item(item)
            );

            break_ix += 1;
        }

        Table::new(
            Border::default(),
            column_breaks,
            column_headers,
            row_headers,
            data_rows
        )
    }

    /// Returns the contents of a table formatted as a string.
    ///
    /// # Arguments
    ///
    /// * `self` - The table to format.
    pub fn format(self: &Table) -> String {
        let mut result: String = String::from("");

        // Format header row
        result.push_str(&self.format_header());

        // Format table body
        result.push_str(&self.format_body());

        result
    }

    /// Returns a vector of column breaks from a format string.
    ///
    /// # Arguments
    ///
    /// * `breaks_format` - The column breaks format string to parse.
    fn parse_column_breaks(
        breaks_format: &str
    ) -> Vec<ColumnBreak> {
        let mut breaks = Vec::new();
        for b in breaks_format.split(' ').collect::<Vec<&str>>() {
            breaks.push(ColumnBreak::from_str(b).unwrap());
        };

        breaks
    }

    /// Formats the table's column headers.
    ///
    /// # Arguments
    ///
    /// * `self` - The table containing the column headers to format.
    fn format_header(
        self: &Table
    ) -> String {
        let mut result: String = String::from("");

        let header_width = self.measure_width();
        
        // Print top border
        result.push_str(&self.border.format_top(header_width));
        result.push_str("\n");

        // Render column header row
        result.push_str(
            &self.column_headers.format(
                &self.border, 
                &self.column_breaks
            )
        );

        // Print horizontal split beneath headers
        result.push_str(&self.border.format_horizontal_split(header_width));
        result.push_str("\n");

        result
    }

    /// Formats the body of a table.
    ///
    /// The specified `width` describes a desired output size and will be the
    ///  maximum size of the formatted output. However, the table may also be
    ///  formatted to a shorter width if there are insufficient column widths
    ///  available to justify the full value.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being formatted.
    /// * `maximum_width` - The maximum render width, in chars.
    fn format_body(
        self: &Table
    ) -> String {
        let render_width = self.measure_width();

        let mut result: String = String::from("");

        // Iterate rows
        for row_ix in 0..self.data_rows.len() {
            let row = &self.data_rows[row_ix];
            result.push_str(
                &row.format(
                    &self.border, 
                    &self.column_breaks
                )
            );
        
            // Print horizontal split beneath all but last row
            if row_ix < self.data_rows.len() - 1 {
                result.push_str(
                    &self.border.format_horizontal_split(render_width));
                result.push_str("\n");
            }
        }

        // Print bottom border at end of table
        result.push_str(&self.border.format_bottom(render_width));
        result.push_str("\n");

        result
    }

    /// Measures the width of a table.
    /// 
    /// Column breaks are used to constrain the render width of columns and 
    ///  are considered along with the content of the header cells.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being measured.
    fn measure_width(
        self: &Table
    ) -> usize {
        let mut header_width = 0;

        // Iterate through the header row
        let mut column_break_ix = 0;
        let content_break = ColumnBreak::Content;
        for cell in self.column_headers.iter() {
            // Get the next column break (if one is available)
            let column_break: &ColumnBreak = 
                if column_break_ix < self.column_breaks.len() {
                    &self.column_breaks[column_break_ix]
                } else {
                    // Use content-width break for additional columns
                    &content_break
                };
            // Calculate the width of this header cell
            header_width += cell.measure_width(column_break);
            // Increment column index
            column_break_ix += 1;
        }

        // Add space for the outer borders
        header_width += 2;

        // Add space for vertical splits separators between columns
        header_width += self.column_headers.len() - 1;

        header_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use crate::content::{Content, Alignment, Wrap};
    use crate::data_item::DataItem;
    use crate::cell;

    /// Tests the simple format table! macro.
    #[test]
    fn table_macro_simple() {
        let table = table!(
            "{B^:12:}" => "Food", "{G^:7:}" => "Count";
            "Fish", "15", "Pizza", "10", "Tomato", "24"
        );

        println!("{}", table.format());

        let expected = "+--------------------+\n|\u{1b}[94m    Food    \u{1b}[0m|\u{1b}[92m Count \u{1b}[0m|\n+--------------------+\n|Fish        |15     |\n+--------------------+\n|Pizza       |10     |\n+--------------------+\n|Tomato      |24     |\n+--------------------+\n";

        assert_eq!(
            table.format(),
            expected
        );
    }
}
