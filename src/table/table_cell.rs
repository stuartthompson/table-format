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
}

