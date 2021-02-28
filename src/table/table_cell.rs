use super::line::Line;
use crate::data_item::DataItem;

/// A table cell represents a single grid rectangle within a table.
/// 
/// Cells belong to a row.
pub struct TableCell {
    pub lines: Vec<Line>,
}

impl TableCell {
    pub fn new() -> TableCell {
        TableCell {
            lines: Vec::new()
        }
    }

    /// Returns a TableCell from a DataItem.
    /// 
    /// # Arguments
    /// 
    /// * `data_item` - The data item from which to build the table cell.
    pub fn from_data_item(
        data_item: DataItem
    ) -> TableCell {
        TableCell {
            lines: data_item.lines
        }
    }

    /// Formats a table cell line.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The table cell containing the line.
    /// * `line_index` - The line index.
    /// * `width` - The format width.
    pub fn format_line(
        self: &TableCell,
        line_index: usize, 
        width: u8
    ) -> String {
        if line_index < self.lines.len() {
            self.lines[line_index].format(width)
        } else {
            String::from("")
        }
    }

    /// Measures the height of this cell that will result from formatting its 
    ///  contents into a specific column width.
    /// 
    ///  # Arguments
    /// 
    /// * `self` - The table cell being measured.
    /// * `column_width` - The column width to measure against.
    pub fn measure_height(
        self: &TableCell,
        column_width: u8
    ) -> u8 {
        let mut height = 0;

        for line in &self.lines {
            height += line.measure_width().div_euclid(column_width) + 1;
        }

        height
    }
}

