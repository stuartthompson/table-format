use crate::content::Content;
use super::table_cell::TableCell;

pub enum ColumnWidth {
    // The column width is fixed
    Fixed(usize),
    // The column is always at least a minimum width
    Minimum(usize)
}

/// Columns represent vertical breakpoints.
pub struct TableColumn {
    pub header_content: TableCell,
    width: ColumnWidth,
}

impl TableColumn {
    pub fn new(
        header_content: Content, 
        width: ColumnWidth
    ) -> TableColumn {
        let mut header_cell: TableCell = TableCell::new();
        // Push content into header cell for title
        header_cell.content_lines.push(header_content);
        TableColumn { 
            header_content: header_cell,
            width,
        }
    }

    /// Returns a new fixed-width table column.
    /// 
    /// # Arguments
    /// 
    /// * `title` - The column title.
    /// * `width` - The fixed width, in chars.
    pub fn fixed(
        header_content: Content,
        width: usize
    ) -> TableColumn {
        TableColumn::new(header_content, ColumnWidth::Fixed(width))
    }

    /// Returns a new minimum-width table column.
    /// 
    /// # Arguments
    /// 
    /// * `title` - The column title.
    /// * `width` - The minimum width of this column.
    pub fn min_width(
        header_content: Content,
        width: usize
    ) -> TableColumn {
        TableColumn::new(header_content, ColumnWidth::Minimum(width))
    }

    /// Returns a new content-width table column.
    /// 
    /// This column will take on the width of its title content.
    /// 
    /// # Arguments
    /// 
    /// * `title` - The column title.
    pub fn content(
        header_content: Content
    ) -> TableColumn {
        let width = &header_content.measure_width();
        TableColumn::new(
            header_content, 
            ColumnWidth::Fixed(*width)
        )
    }

    /// Measures the width of the content of a column.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The column being measured.
    pub fn measure_content_width(
        self: &TableColumn
    ) -> usize {
        let mut longest_line = 0;
        for line in &self.header_content.content_lines {
            let line_width = line.measure_width();
            if line_width > longest_line {
                longest_line = line_width;
            }
        }
        longest_line
    }

    /// Gets the minimum number of characters needed to render this column.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The column to measure.
    pub fn measure_width(
        self: &TableColumn
    ) -> usize {
        match self.width {
            // Fixed width + 2 for border chars
            ColumnWidth::Fixed(fixed_width) => fixed_width,
            ColumnWidth::Minimum(min_width) => {
                // Use min-width if content is shorter
                let content_width = self.measure_content_width();
                if content_width < min_width {
                    min_width
                } else {
                    content_width
                }
            }
        }
    }

    /// Measures the height of a column
    /// 
    /// # Arguments
    /// 
    /// * `self` - The column to measure.
    pub fn measure_height(
        self: &TableColumn
    ) -> usize {
        self.header_content.content_lines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use crate::content::{Content, Alignment, Wrap};

    #[test]
    fn test_measure_width_fixed() {
        let content = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        let column = TableColumn::fixed(content, 10);

        let measured_width = column.measure_width();

        assert_eq!(measured_width, 10);
    }

    #[test]
    fn test_measure_width_min_width_shorter_content() {
        let content = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );
        
        let column = TableColumn::min_width(content, 10);

        let measured_width = column.measure_width();

        assert_eq!(measured_width, 10);
    }

    #[test]
    fn test_measure_width_min_width_longer_content() {
        let content = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        let column = TableColumn::min_width(content, 2);

        let measured_width = column.measure_width();

        assert_eq!(measured_width, 4);
    }

    #[test]
    fn test_measure_width_min_width_same_length_content() {
        let content = Content::new(
            String::from("test"),
            Color::White,
            Alignment::Center,
            Wrap::NoWrap
        );

        let column = TableColumn::min_width(content, 4);

        let measured_width = column.measure_width();

        assert_eq!(measured_width, 4);
    }
}