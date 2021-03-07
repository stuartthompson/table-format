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

#[allow(unused_macros)]
#[macro_export]
macro_rules! row {
    ($( $style:expr => $content:expr ),*) => {
        {
            let mut tr: TableRow = TableRow::new();
            $( tr.add_cell(crate::cell!($style, $content)); )*
            tr
        }
    };
    ($style:expr, $($content:expr),*) => {
        {
            let mut tr: TableRow = TableRow::new();
            $( tr.add_cell(crate::cell!($style, $content)); )*
            tr
        }
    };
}

/// Table rows represent horizontal breakpoints.
#[derive(Debug)]
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

    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
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
    /// * `border` - The table border.
    /// * `column_breaks` - The breakpoints at which to wrap or truncate.
    pub fn format(
        self: &TableRow,
        border: &Border,
        column_breaks: &Vec<ColumnBreak>
    ) -> String {
        let mut result: String = String::from("");

        let row_height = self.measure_height(column_breaks);

        // Get content iterators for each cell
        let mut content_iterators = Vec::new();
        for cell_ix in 0..self.cells.len() {
            let cell = &self.cells[cell_ix];
            let column_break = &column_breaks[cell_ix];
            content_iterators.push(cell.get_iterator(&column_break));    
        }

        // Iterate the number of lines
        let content_break = ColumnBreak { width: BreakWidth::Content };
        for _line_ix in 0..row_height {
            // Left border
            result.push_str(&border.format_left());
            // Write the contents for the current line of the cell
            for cell_ix in 0..self.cells.len() {
                let cell = &self.cells[cell_ix];
                let column_break: &ColumnBreak =
                    if cell_ix < column_breaks.len() {
                        &column_breaks[cell_ix]
                    } else {
                        &content_break
                    };
                result.push_str(
                    &match content_iterators[cell_ix].next() {
                        Some(content) => format!("{}", content),
                        None => {
                            // No more lines so fill height with empty space
                            let cell_width = cell.measure_width(column_break);
                            format!("{}", (0..cell_width)
                                .map(|_| " ")
                                .collect::<String>())
                        }
                    }
                );
                // Vertical split (except for final column)
                if cell_ix < column_breaks.len() - 1 {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_macro_style_per_cell() {
        assert_eq!(
            format!("{:?}", row!("{c^}" => "Head 1", "{G-r>}" => "Head 2")),
            format!("{:?}", TableRow::from(
                vec!(
                    crate::cell!("{c^}", "Head 1"),
                    crate::cell!("{G-r>}", "Head 2")
                )
            ))
        );
    }

    #[test]
    fn test_row_macro_common_style() {
        assert_eq!(
            format!("{:?}", row!("{c^}", "Text 1", "Text 2", "Text 3")),
            format!("{:?}", TableRow::from(
                vec!(
                    crate::cell!("{c^}", "Text 1"),
                    crate::cell!("{c^}", "Text 2"),
                    crate::cell!("{c^}", "Text 3")
                )
            ))
        );
    }
}