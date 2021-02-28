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
}

