use super::border::Border;
use super::table_cell::TableCell;
use super::table_column::TableColumn;

/// Table rows represent horizontal breakpoints.
pub struct TableRow {
    pub cells: Vec<TableCell>
}

impl TableRow {
    pub fn new() -> TableRow {
        TableRow {
            cells: Vec::new()
        }
    }

    /// Formats a table row.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The table row to format.
    /// * `maximum_width` - The maximum render width.
    pub fn format(
        self: &TableRow,
        border: &Border,
        columns: &Vec<TableColumn>,
        maximum_width: u8
    ) -> String {
        let mut result: String = String::from("");

        let row_height = self.measure_height(columns);

        // Iterate the number of lines
        for line_ix in 0..row_height {
            // Left border
            result.push_str(&border.format_left());
            // Write the contents for the current line of the cell
            for cell_ix in 0..self.cells.len() {
                let cell = &self.cells[cell_ix];
                let column = &columns[cell_ix];
                result.push_str(
                    &cell
                        .format_line(
                            line_ix as usize, column.measure_width())
                );
                // Vertical split (except for final column)
                if cell_ix < columns.len() - 1 {
                    result.push_str(&border.format_vertical_split());
                }
            }
            // Right border
            result.push_str(&border.format_right());
            result.push_str("\n");
        }

        result
    }

    /// Measures the height of a table row.
    ///
    /// # Arguments
    ///
    /// * `self` - The table row being measured.
    /// * `columns` - The columns used to format the cells for this row.
    fn measure_height(
        self: &TableRow,
        columns: &Vec<TableColumn>,
    ) -> u8 {
        let mut tallest_height = 0;

        // Iterate the columns and measure cells based upon their column
        for column_ix in 0..columns.len() {
            let column = &columns[column_ix];

            // Guard against more columns than cells
            if column_ix < self.cells.len() {
                let cell = &self.cells[column_ix];
                let cell_height = cell.measure_height(column.measure_width());
                if cell_height > tallest_height {
                    tallest_height = cell_height;
                }
            }
        }

        tallest_height
    }

}
