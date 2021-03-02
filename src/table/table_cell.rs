use crate::content::{Content, ContentIterator};
use super::column_break::{ColumnBreak, BreakWidth};
use crate::data_item::DataItem;

pub struct TableCellContentIterator<'a> {
    content: &'a Vec<Content>,
    current_content_iterator: ContentIterator,
    current_line_ix: usize,
    width: usize,
    target_height: usize,
    current_height: usize,
}

impl<'a> Iterator for TableCellContentIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let line = 
            if self.current_line_ix < self.content.len() {
                // Get the next line from the current content iterator
                match self.current_content_iterator.next() {
                    Some(content) => {
                        Some(content)
                    },
                    None => {
                        // Go to the next line
                        self.current_line_ix += 1;

                        // If there are more lines, get iterator for next line
                        if self.current_line_ix < self.content.len() {
                            self.current_content_iterator = 
                                self.content[self.current_line_ix].get_iterator(self.width);

                            // Get the line from the iterator
                            self.current_content_iterator.next()
                        } else {
                            // No more lines to get
                            None
                        }
                    }
                }
            } else {
                None
            };

        match line {
            Some(content) => {
                Some(content)
            } ,
            None => {
                if self.current_height < self.target_height {
                    // An empty line of spaces the width of the column
                    let result = 
                        (0..self.width)
                            .map(|_| " ")
                            .collect::<String>();
                    self.current_height += 1;
                    Some(result)
                } else {
                    None
                }
            }
        }
    }
}

/// A table cell represents a single grid rectangle within a table.
/// 
/// Cells belong to a row.
pub struct TableCell {
    pub contents: Vec<Content>
}

impl TableCell {
    pub fn new() -> TableCell {
        TableCell {
            contents: Vec::new()
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
            contents: data_item.lines
        }
    }

    /// Returns the next formatted line of content from this table cell.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The table cell containing the line.
    /// * `width` - The format width.
    pub fn get_iterator(
        self: &TableCell,
        column_break: &ColumnBreak
    ) -> TableCellContentIterator {
        // Determine the render width of this cell
        let cell_width = self.measure_width(column_break);

        TableCellContentIterator {
            content: &self.contents,
            current_content_iterator: self.contents[0].get_iterator(cell_width),
            current_line_ix: 0,
            width: cell_width,
            target_height: self.measure_height(column_break),
            current_height: 0
        }
    }

    /// Measures the height needed for this cell when formatting its contents 
    ///  into a specific column width.
    /// 
    ///  # Arguments
    /// 
    /// * `self` - The table cell being measured.
    /// * `column_width` - The column width to measure against.
    pub fn measure_height(
        self: &TableCell,
        column_break: &ColumnBreak,
    ) -> usize {
        let mut height = 0;

        // Determine the render width of this cell
        let cell_width = self.measure_width(column_break);

        for line in &self.contents {
            let line_width = line.measure_width();
            // If line fits within column then line height is 1
            if !line.will_wrap() || (line_width <= cell_width) {
                height += 1
            } else {
                // Determine how many lines are needed when content is wrapped
                height += line_width.div_euclid(cell_width) + 1;
            }
        }

        height
    }

    /// Measures the width of this cell.
    ///
    /// # Arguments
    /// 
    /// * `self` - The table cell being measured.
    /// * `column_break` - The column break for this cell.
    pub fn measure_width(
        self: &TableCell,
        column_break: &ColumnBreak,
    ) -> usize {
        match column_break.width {
            BreakWidth::Fixed(fixed) => fixed,
            BreakWidth::Minimum(minimum_width) => {
                let content_width = self.measure_content_width();
                if minimum_width > content_width {
                    minimum_width
                } else {
                    content_width
                }
            },
            BreakWidth::Content => {
                self.measure_content_width()
            }
        }
    }

    /// Returns the width of the longest content item in this cell.
    /// 
    /// This measure ignores wrapping or truncation and returns the raw width 
    ///  of the longest content item.
    fn measure_content_width(
        self: &TableCell
    ) -> usize {
        let mut largest = 0;
        for content in &self.contents {
            let content_width = content.measure_width();
            if content_width > largest {
                largest = content_width;
            } 
        }

        largest  
    }
}
