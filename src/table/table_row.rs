use super::table_cell::TableCell;

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
}
