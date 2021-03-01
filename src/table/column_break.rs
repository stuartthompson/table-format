pub enum BreakWidth {
    // The column width is fixed
    Fixed(usize),
    // The column is always at least a minimum width
    Minimum(usize),
    // The column takes on the width of its header content
    Content,
}

/// Column breaks describe where content should be divided
pub struct ColumnBreak {
    pub width: BreakWidth
}