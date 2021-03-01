use super::border::Border;
use super::column_break::{ColumnBreak, BreakWidth};
use super::table_cell::TableCell;

pub struct CellIterator<'a> {
    cells: &'a Vec<TableCell>,
    current_cell_ix: usize
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = &'a TableCell;

    fn next(&mut self) -> Option<&'a TableCell> {
        if self.current_cell_ix < self.cells.len() {
            let cell: &TableCell = &self.cells[self.current_cell_ix];
            self.current_cell_ix += 1;
            Some(cell)
        } else {
            None
        }
    }
}

/// Table rows represent horizontal breakpoints.
pub struct TableRow {
    cells: Vec<TableCell>
}

impl TableRow {
    pub fn new() -> TableRow {
        TableRow {
            cells: Vec::new()
        }
    }

    pub fn from(
        cells: Vec<TableCell>
    ) -> TableRow {
        TableRow { cells }
    }

    pub fn iter(
        self: &TableRow,
    ) -> CellIterator {
        CellIterator {
            cells: &self.cells,
            current_cell_ix: 0
        }
    }

    pub fn len(self: &TableRow) -> usize { 
        self.cells.len()
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
        column_breaks: &Vec<ColumnBreak>,
        maximum_width: usize
    ) -> String {
        let mut result: String = String::from("");

        let row_height = self.measure_height(column_breaks);

        // Get content iterators for each cell
        let mut content_iterators = Vec::new();
        for cell_ix in 0..self.cells.len() {
            let cell = &self.cells[cell_ix];
            let column_break = &column_breaks[cell_ix];
            content_iterators.push(cell.get_iterator(column_break.measure_width()));
        }

        // Iterate the number of lines
        for line_ix in 0..row_height {
            // Left border
            result.push_str(&border.format_left());
            // Write the contents for the current line of the cell
            for cell_ix in 0..self.cells.len() {
                let cell = &self.cells[cell_ix];
                let column = &columns[cell_ix];
                result.push_str(
                    &match content_iterators[cell_ix].next() {
                        Some(content) => format!("{}", content),
                        None => {
                            // No more lines so fill height with empty space
                            format!("{}", (0..column.measure_width())
                                .map(|_| " ")
                                .collect::<String>())
                        }
                    }
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
    pub fn measure_height(
        self: &TableRow,
        column_breaks: &Vec<ColumnBreak>,
    ) -> usize {
        let mut tallest_height = 0;

        // Iterate the row cells and measure based upon supplied column breaks
        let column_break_ix = 0;
        let content_break = ColumnBreak { width: BreakWidth::Content };
        for cell in &self.cells {
            // Get the next column break (if one is available)
            let column_break: &ColumnBreak = 
                if column_break_ix < column_breaks.len() {
                    &column_breaks[column_break_ix]
                } else {
                    // Use content-width break for additional columns
                    &content_break
                };
            let cell_height = cell.measure_height(column_break);
            if cell_height > tallest_height {
                tallest_height = cell_height;
            }
        }

        tallest_height
    }
}
