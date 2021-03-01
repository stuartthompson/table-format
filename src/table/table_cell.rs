use crate::content::{Content, ContentIterator};
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
    pub content_lines: Vec<Content>
}

impl TableCell {
    pub fn new() -> TableCell {
        TableCell {
            content_lines: Vec::new()
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
            content_lines: data_item.lines
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
        width: usize
    ) -> TableCellContentIterator {
        TableCellContentIterator {
            content: &self.content_lines,
            current_content_iterator: self.content_lines[0].get_iterator(width),
            current_line_ix: 0,
            width: width,
            target_height: self.measure_height(width),
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
        column_width: usize
    ) -> usize {
        let mut height = 0;

        for line in &self.content_lines {
            let line_width = line.measure_width();
            // If line fits within column then line height is 1
            if !line.will_wrap() || (line_width <= column_width) {
                height += 1
            } else {
                // Determine how many lines are needed when content is wrapped
                height += line_width.div_euclid(column_width) + 1;
            }
        }

        height
    }
}
