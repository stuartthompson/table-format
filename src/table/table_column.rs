use super::table_cell::TableCell;
use super::line::Line;

pub enum ColumnWidth {
    // The width of the column is determined by the length of the longest line 
    //  in the header.
    Content,
    // The column width is fixed
    Fixed(u8),
    // The column is always at least a minimum width
    Minimum(u8)
}

/// Columns represent vertical breakpoints.
pub struct TableColumn {
    pub header_content: TableCell,
    width: ColumnWidth,
}

impl TableColumn {
    pub fn new(header_title: String, width: ColumnWidth) -> TableColumn {
        let mut header_cell: TableCell = TableCell::new(); 
        header_cell.lines.push(Line::from(header_title));
        TableColumn { 
            header_content: header_cell,
            width,
        }
    }

    pub fn fixed(
        header_title: String,
        width: u8
    ) -> TableColumn {
        TableColumn::new(header_title, ColumnWidth::Fixed(width))
    }

    pub fn min_width(
        header_title: String,
        width: u8
    ) -> TableColumn {
        TableColumn::new(header_title, ColumnWidth::Minimum(width))
    }

    pub fn content_width(
        self: &TableColumn
    ) -> u8 {
        let mut longest_line = 0;
        for line in &self.header_content.lines {
            let line_width = line.measure_width();
            if line_width > longest_line {
                longest_line = line_width;
            }
        }
        longest_line as u8
    }

    /// Gets the minimum number of characters needed to render this column.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The column to measure.
    pub fn measure_width(
        self: &TableColumn
    ) -> u8 {
        match self.width {
            // Fixed width + 2 for border chars
            ColumnWidth::Fixed(fixed_width) => fixed_width,
            ColumnWidth::Minimum(min_width) => {
                // Either min width or width of content
                //  (whichever is shorter)
                let content_width = self.content_width();
                if min_width < content_width {
                    min_width
                } else {
                    content_width
                }
            },
            ColumnWidth::Content => self.content_width()
        }
    }

    /// Measures the height of a column
    /// 
    /// # Arguments
    /// 
    /// * `self` - The column to measure.
    pub fn measure_height(
        self: &TableColumn
    ) -> u8 {
        self.header_content.lines.len() as u8
    }
}