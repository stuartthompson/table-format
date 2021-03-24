use super::border::Border;
use super::cell::Cell;
use crate::content::{CellWidth};

pub struct CellIterator<'a> {
    cells: &'a Vec<Cell>,
    current_cell_ix: usize
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        if self.current_cell_ix < self.cells.len() {
            let cell: &Cell = &self.cells[self.current_cell_ix];
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
    ( $($style:expr => $content:expr),* ) => {
        {
            let mut r: Row = Row::new();
            $( r.add_cell(crate::cell!($style, $content)); )*
            r
        }
    };
    ( $style:expr, $($content:expr),* ) => {
        {
            let mut r: Row = Row::new();
            $( r.add_cell(crate::cell!($style, $content)); )*
            r
        }
    };
}

/// Table rows represent horizontal breakpoints.
#[derive(Debug)]
pub struct Row {
    cells: Vec<Cell>
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Row {
    #[must_use]
    pub fn new() -> Row {
        Row {
            cells: Vec::new()
        }
    }

    #[must_use]
    pub fn from(
        cells: Vec<Cell>
    ) -> Row {
        Row { cells }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    #[must_use]
    pub fn iter(
        self: &Row,
    ) -> CellIterator {
        CellIterator {
            cells: &self.cells,
            current_cell_ix: 0
        }
    }

    #[must_use]
    pub fn len(self: &Row) -> usize {
        self.cells.len()
    }

    #[must_use]
    pub fn is_empty(self: &Row) -> bool {
        self.len() == 0
    }

    /// Formats a table row.
    ///
    /// # Arguments
    ///
    /// * `self` - The table row to format.
    /// * `border` - The table border.
    /// * `column_breaks` - The breakpoints at which to wrap or truncate.
    #[must_use]
    #[allow(clippy::option_if_let_else)]
    pub fn format(
        self: &Row,
        border: &Border,
        column_breaks: &[CellWidth]
    ) -> String {
        let mut result: String = String::from("");

        let row_height = self.measure_height(column_breaks);

        // Get content iterators for each cell
        let mut content_iterators = Vec::new();
        for (cell_ix, cell) in self.cells.iter().enumerate() {
            let column_break = &column_breaks[cell_ix];
            content_iterators.push(cell.get_iterator(&column_break));
        }

        // Iterate the number of lines
        let content_break = CellWidth::default();
        for _line_ix in 0..row_height {
            // Left border
            result.push_str(&border.format_left());
            // Write the contents for the current line of the cell
            for cell_ix in 0..self.cells.len() {
                let cell = &self.cells[cell_ix];
                let column_break: &CellWidth =
                    if cell_ix < column_breaks.len() {
                        &column_breaks[cell_ix]
                    } else {
                        &content_break
                    };
                result.push_str(
                    &if let Some(content) = content_iterators[cell_ix].next() {
                        content.to_string()
                    } else {
                        // No more lines so fill height with empty space
                        let cell_width = cell.measure_width(column_break);
                        (0..cell_width)
                            .map(|_| " ")
                            .collect::<String>()
                    }
                );
                // Vertical split (except for final column)
                if cell_ix < column_breaks.len() - 1 {
                    result.push_str(&border.format_vertical_split());
                }
            }
            // Right border
            result.push_str(&border.format_right());
            result.push('\n');
        }

        result
    }

    /// Measures the height of a table row.
    ///
    /// # Arguments
    ///
    /// * `self` - The table row being measured.
    /// * `columns` - The columns used to format the cells for this row.
    #[must_use]
    pub fn measure_height(
        self: &Row,
        column_breaks: &[CellWidth],
    ) -> usize {
        let mut tallest_height = 0;

        // Iterate the row cells and measure based upon supplied column breaks
        let column_break_ix = 0;
        let content_break = CellWidth::Content;
        for cell in &self.cells {
            // Get the next column break (if one is available)
            let column_break: &CellWidth = 
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
            format!("{:?}", Row::from(
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
            format!("{:?}", Row::from(
                vec!(
                    crate::cell!("{c^}", "Text 1"),
                    crate::cell!("{c^}", "Text 2"),
                    crate::cell!("{c^}", "Text 3")
                )
            ))
        );
    }
}
