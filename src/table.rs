mod border;
mod table_cell;
mod table_column;
mod table_row;

use super::data_item::DataItem;
use border::Border;
use table_cell::TableCell;
pub use table_column::TableColumn;
use table_row::TableRow;

pub struct Table {
    border: Border,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
}

impl Table {
    // Returns an empty table
    pub fn new() -> Table {
        Table {
            border: Border::default(),
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Builds a table from a data source and formats using a set of 
    ///  supplied columns.
    /// 
    /// The columns are used to understand where new rows should occur.
    ///
    /// # Arguments
    ///
    /// * `source` - The data source to build the table from.
    /// * `columns` - Columns describing how the data is structured.
    pub fn from(
        source: &mut dyn Iterator<Item = DataItem>, 
        columns: Vec<TableColumn>
    ) -> Table {
        let mut table = Table::new();
        table.columns = columns;

        // Track current column index
        let mut next_column_index: usize = 0;

        // Add first row to table
        let mut row_index: usize = 0;
        table.rows.push(TableRow::new());

        // Iterate the data source and build the table rows
        loop {
            // Get next data item
            match source.next() {
                Some(data_item) => {
                    // Is a new row needed?
                    if next_column_index == table.columns.len() {
                        // Add a new row to the table
                        table.rows.push(TableRow::new());
                        row_index = row_index + 1;
                        // Reset next column index
                        next_column_index = 0;
                    }

                    // Build a cell for this data item
                    let cell = TableCell::from_data_item(data_item);

                    // Append cell
                    table.rows[row_index].cells.push(cell);

                    // Advance column index (wraps to 0 when out of columns)
                    next_column_index += 1;
                }
                None => {
                    break;
                }
            }
        }

        table
    }

    /// Adds a row to a table.
    ///
    /// # Arguments
    ///
    /// * `self` - The table to add the row to.
    /// * `row` - The row to add.
    pub fn add_row(self: &mut Table, row: TableRow) {
        self.rows.push(row);
    }

    /// Returns the contents of a table formatted as a string.
    ///
    /// # Arguments
    ///
    /// * `self` - The table to format.
    /// * `width` - The width in chars at which to wrap columns.
    pub fn format(self: &Table, width: u8) -> String {
        let mut result: String = String::from("");

        // Format table headers
        result.push_str(&self.format_header(width));

        // Format table body
        result.push_str(&self.format_body());

        result
    }

    /// Formats the table's column headers.
    ///
    /// # Arguments
    ///
    /// * `self` - The table containing the column headers to format.
    /// * `width` - The width in chars at which to wrap columns.
    fn format_header(self: &Table, width: u8) -> String {
        let mut result: String = String::from("");

        let header_width = self.measure_width();
        let header_height = self.measure_header_height();

        // Print top border
        result.push_str(&self.border.format_top(header_width));
        result.push_str("\n");

        // Iterate the number of lines
        for line_ix in 0..header_height {
            // Left border
            result.push_str(&self.border.format_left());
            // Write the column headers for this line
            for col_ix in 0..self.columns.len() {
                let col = &self.columns[col_ix];
                result.push_str("HEADER");
                // Vertical split (except for final column)
                if col_ix < self.columns.len() - 1 {
                    result.push_str(&self.border.format_vertical_split());
                }
            }
            // Right border
            result.push_str(&self.border.format_right());
            result.push_str("\n");
        }

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
        for row_ix in 0..self.rows.len() {
            let row = &self.rows[row_ix];
            result.push_str(&row.format(&self.border, &self.columns, render_width));
        
            // Print horizontal split beneath all but last row
            if row_ix < self.rows.len() - 1 {
                result.push_str(&self.border.format_horizontal_split(render_width));
                result.push_str("\n");
            }
        }

        // Print bottom border at end of table
        result.push_str(&self.border.format_bottom(render_width));
        result.push_str("\n");

        result
    }

    /// Measures the width of a table based upon its columns.
    ///
    /// The table columns each describe their widths. This is used to format
    ///  the final output width of the table.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being measured.
    fn measure_width(self: &Table) -> usize {
        let mut header_width = 0;

        // Sum the widths of the header columns
        for col in &self.columns {
            header_width += col.measure_width()
        }

        // Add space for the outer borders
        header_width += 2;

        // Add space for vertical splits separators between columns
        header_width += self.columns.len() - 1;

        header_width
    }

    /// Measures the height of a table's column headers.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being measured.
    fn measure_header_height(self: &Table) -> usize {
        let mut tallest_height = 0;

        for col in &self.columns {
            let col_height = col.measure_height();
            if col_height > tallest_height {
                tallest_height = col_height
            }
        }

        tallest_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use crate::content::{Content, Alignment, Wrap};

    #[test]
    fn measure_header_one_column() {
        let mut table = Table::new();

        let header = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        table
            .columns
            .push(TableColumn::fixed(header, 15));

        // Expect 15 chars for column, 2 for outer border chars
        let expected_width = 17;

        assert_eq!(table.measure_width(), expected_width);
    }

    #[test]
    fn measure_header_two_columns() {
        let mut table = Table::new();

        let header1 = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        let header2 = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        table
            .columns
            .push(TableColumn::fixed(header1, 15));
        table
            .columns
            .push(TableColumn::fixed(header2, 15));

        // Expect 33 chars. 2 x 15 columns + 2 for outer border, 1 for split
        let expected_width = 33;

        assert_eq!(table.measure_width(), expected_width);
    }
}
