use crate::content::{Content, ContentIterator, ContentStyle, CellWidth};
use crate::data_item::DataItem;
use std::clone::Clone;

pub struct TableCellContentIterator<'a> {
    content: &'a Vec<Content>,
    current_content_iterator: ContentIterator,
    current_line_ix: usize,
    base_style: ContentStyle,
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
                                self.content[self.current_line_ix].get_iterator(self.base_style.clone(), self.width);

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

#[allow(unused_macros)]
#[macro_export]
macro_rules! cell_content {
    ($($out:tt)*) => {
        vec!($($out)*)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! cell {
    ($style:expr, $($content:tt)*) => {
        Cell::from_styled_content(
            $style, 
            crate::cell_content!($($content)*)
        );
    }
}

/// A table cell represents a single grid rectangle within a table.
/// 
/// Cells belong to a row.
#[derive(Debug)]
pub struct Cell {
    contents: Vec<Content>,
    base_style: ContentStyle,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            contents: Vec::new(),
            base_style: ContentStyle::default(),
        }
    }

    pub fn new(
        contents: Vec<Content>,
        base_style: ContentStyle,
    ) -> Cell {
        Cell {
            contents,
            base_style
        }
    }

    pub fn from_styled_content(
        format: &str,
        contents: Vec<&str>,
    ) -> Cell {
        // Split the format string into style tokens
        let styles: Vec<&str> = format.split(' ').collect();
        let mut table_cell = Cell::empty();

        // Iterate the contents
        for (style_ix, content) in contents.into_iter().enumerate() {
            // Get next style (use default if no more styles provided)
            let style = 
                if style_ix < styles.len() { 
                    ContentStyle::from_format(styles[style_ix]) }
                else { ContentStyle::default() };

            // Add the new styled content
            table_cell.contents.push(
                Content::new(content.to_string(), Some(style)));
        }
        table_cell
    }

    /// Returns a TableCell from a DataItem.
    /// 
    /// # Arguments
    /// 
    /// * `data_item` - The data item from which to build the table cell.
    /// * `base_style` - The base style to apply to the cell contents.
    pub fn from_data_item(
        data_item: &DataItem,
        base_style: ContentStyle,
    ) -> Cell {
        Cell::new(
            data_item.lines.to_vec(),
            base_style,
        )
    }

    /// Returns the column break specified in the first content line of the 
    /// cell.
    /// 
    /// This is used to determine the column break for cells used in the table 
    /// header row.
    pub fn get_cell_width(
        self: &Cell
    ) -> CellWidth {
        if self.contents.is_empty() {
            CellWidth::default() }
        else {
            match &self.contents[0].style {
                Some(style) => style.width.clone(),
                None => CellWidth::default()
            }
        }
    }

    /// Returns the next formatted line of content from this table cell.
    /// 
    /// # Arguments
    /// * `self` - The table cell containing the line.  * `width` - The format width.
    pub fn get_iterator(
        self: &Cell,
        column_break: &CellWidth
    ) -> TableCellContentIterator {
        // Determine the render width of this cell
        let cell_width = self.measure_width(column_break);

        TableCellContentIterator {
            content: &self.contents,
            current_content_iterator: 
                self.contents[0].get_iterator(self.base_style.clone(), cell_width),
            current_line_ix: 0,
            base_style: self.base_style.clone(),
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
        self: &Cell,
        column_break: &CellWidth,
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
        self: &Cell,
        column_break: &CellWidth,
    ) -> usize {
        match column_break {
            CellWidth::Fixed(fixed) => *fixed,
            CellWidth::Minimum(minimum_width) => {
                let content_width = self.measure_content_width();
                if minimum_width > &content_width {
                    *minimum_width
                } else {
                    content_width
                }
            },
            CellWidth::Content => {
                self.measure_content_width()
            }
        }
    }

    /// Returns the width of the longest content item in this cell.
    /// 
    /// This measure ignores wrapping or truncation and returns the raw width 
    ///  of the longest content item.
    fn measure_content_width(
        self: &Cell
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

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use crate::content::{Alignment, Wrap};

    #[test]
    fn test_table_cell_macro() {
        let tc = cell!("{r<;} {G-b^}", "testing", "this");

        assert_eq!(tc.contents.len(), 2);

        assert_eq!(
            format!("{:?}", tc.contents[0]),
            format!("{:?}", Content::new(
                "testing".to_string(),
                Some(ContentStyle::new(
                    Some(Color::Red),
                    None,
                    Alignment::Left,
                    Wrap::Wrap,
                    CellWidth::Content
                ))
            ))
        );
    }
}
