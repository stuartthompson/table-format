mod border;
pub mod line;
mod table_cell;
mod table_column;
mod table_row;

use border::Border;
use table_cell::TableCell;
pub use table_column::TableColumn;
use table_row::TableRow;
use super::data_item::DataItem;

pub struct Table {
    border: Border,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>
}

impl Table {
    // Returns an empty table 
    pub fn new() -> Table {
        Table {
            border: Border::default(),
            columns: Vec::new(),
            rows: Vec::new()
        }
    }

    /// Builds a table from a data source
    /// 
    /// # Arguments
    /// 
    /// * `source` - The data source to build the table from.
    /// * `columns` - Columns describing how the data is structured.
    pub fn from(
        source: &mut dyn Iterator<Item = DataItem>,
        columns: Vec<TableColumn>,
    ) -> Table {
        let mut table = Table::new();
        table.columns = columns;

        // Track current column index
        let mut column_index: usize = 0;

        // Track current row index
        let mut row_index: usize = 0;

        // Add first row to table
        table.rows.push(TableRow::new());

        // Iterate the data source and build the table rows
        loop {
            // Get next data item
            match source.next() {
                Some(data_item) => {
                    // Build a cell for this data item
                    let cell = TableCell::from_data_item(data_item);

                    // Append cell
                    table.rows[row_index].cells.push(cell);
                },
                None => {
                    break;
                }
            }

            // Advance column index (wraps to 0 when out of columns)
            if column_index < table.columns.len() {
                column_index = column_index + 1;
            } 
            else {
                column_index = 0;
                // Add a new row to the table
                table.rows.push(TableRow::new());
                row_index = row_index + 1;
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
        
        // Print table headers
        result.push_str(&self.format_header(width));

        // Iterate the rows in the table
        for row in &self.rows {
            // Iterate columns
            for col in &self.columns {

            }
        }

        result
    }

    /// Formats the column headers for a table.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The table containing the column headers to format.
    /// * `width` - The width in chars at which to wrap columns. 
    fn format_header(
        self: &Table,
        width: u8
    ) -> String {
        let mut result: String = String::from("");

        let current_width = 0;
        let cols_rendered = 0;

        let header_width = Table::measure_header(&self.columns);
        
        // Print top border
        result.push_str(&format!("{}\n", &self.border.format_top(width)));

        // Iterate the columns
        for col in &self.columns {
            // Render the lines of the column header
            for line in &col.header_content.lines {
                result.push_str(&line.format(col.measure_width()));
            }
        }
        result.push_str("\n");

        // Print debug line lengths
        for col in &self.columns {
            result.push_str(&format!("Length: {}", col.measure_width()));
        }
        result.push_str("\n");

        // Print horizontal split beneath headers
        result.push_str(&format!("{}\n", &self.border.format_bottom(width)));

        result
    }

    /// Returns the width required to draw table columns
    fn measure_header(
        columns: &Vec<TableColumn>
    ) -> u8 {
        let mut consumed_width = 0;
        
        for col in columns {
            consumed_width += col.measure_width()
        }

        consumed_width
    }

    /// Measures the height of a table's column headers.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The table being measured.
    fn measure_header_height(
        self: &Table
    ) -> u8 {
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