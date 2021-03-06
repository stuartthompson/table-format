mod border;
pub mod column_break;
pub mod table_row;
pub mod table_cell;

use border::Border;
use column_break::{ColumnBreak, BreakWidth};
use table_cell::TableCell;
use table_row::TableRow;
use super::content::Content;
use super::data_item::DataItem;
use super::VecDataSource;

pub struct Table {
    border: Border,
    column_breaks: Vec<ColumnBreak>,
    column_headers: TableRow,
    row_headers: Vec<TableCell>,
    data_rows: Vec<TableRow>
}

impl Table {
    // Returns an empty table
    pub fn new() -> Table {
        Table {
            border: Border::default(),
            column_breaks: Vec::new(),
            column_headers: TableRow::new(),
            row_headers: Vec::new(),
            data_rows: Vec::new(),
        }
    }

    /// Builds a table from a data source and formats using a set of 
    ///  supplied columns.
    /// 
    /// The columns define vertical breaks used determine when to wrap or 
    ///  truncate content.
    ///
    /// # Arguments
    ///
    /// * `source` - The data source to build the table from.
    /// * `columns` - Columns describing how the data is structured.
    pub fn from(
        column_breaks: Vec<ColumnBreak>,
        column_headers: TableRow,
        row_headers: Vec<TableCell>,
        data_rows: Vec<TableRow>,
    ) -> Table {
        Table {
            border: Border::default(),
            column_breaks,
            column_headers,
            row_headers,
            data_rows
        }
    }

    pub fn from_vec_data_source(
        column_breaks: Vec<ColumnBreak>,
        column_headers: TableRow,
        row_headers: Vec<TableCell>,
        data_source: VecDataSource<&str>,
    ) -> Table {

        let mut data_rows = Vec::new();
        
        // Create a new row
        let mut row_ix = 0;
        data_rows.push(TableRow::new());

        let mut break_ix = 0;

        for item in data_source {
            // Add a new row if needed
            if break_ix == column_breaks.len() {
                break_ix = 0;
                data_rows.push(TableRow::new());
                row_ix += 1;
            }

            data_rows[row_ix].add_cell(
                TableCell::from_data_item(item)
            );

            break_ix += 1;
        }

        Table {
            border: Border::default(),
            column_breaks,
            column_headers,
            row_headers,
            data_rows
        }
    }

    /// Returns the contents of a table formatted as a string.
    ///
    /// # Arguments
    ///
    /// * `self` - The table to format.
    pub fn format(self: &Table) -> String {
        let mut result: String = String::from("");

        // Format header row
        result.push_str(&self.format_header());

        // Format table body
        result.push_str(&self.format_body());

        result
    }

    /// Formats the table's column headers.
    ///
    /// # Arguments
    ///
    /// * `self` - The table containing the column headers to format.
    fn format_header(
        self: &Table
    ) -> String {
        let mut result: String = String::from("");

        let header_width = self.measure_width();
        
        // Print top border
        result.push_str(&self.border.format_top(header_width));
        result.push_str("\n");

        // Render column header row
        result.push_str(
            &self.column_headers.format(
                &self.border, 
                &self.column_breaks
            )
        );

        // Print horizontal split beneath headers
        result.push_str(&self.border.format_horizontal_split(header_width));
        result.push_str("\n");

        result
    }

    /// Formats the body of a table.
    ///
    /// The specified `width` describes a desired output size and will be the
    ///  maximum size of the formatted output. However, the table may also be
    ///  formatted to a shorter width if there are insufficient column widths
    ///  available to justify the full value.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being formatted.
    /// * `maximum_width` - The maximum render width, in chars.
    fn format_body(
        self: &Table
    ) -> String {
        let render_width = self.measure_width();

        let mut result: String = String::from("");

        // Iterate rows
        for row_ix in 0..self.data_rows.len() {
            let row = &self.data_rows[row_ix];
            result.push_str(
                &row.format(
                    &self.border, 
                    &self.column_breaks
                )
            );
        
            // Print horizontal split beneath all but last row
            if row_ix < self.data_rows.len() - 1 {
                result.push_str(
                    &self.border.format_horizontal_split(render_width));
                result.push_str("\n");
            }
        }

        // Print bottom border at end of table
        result.push_str(&self.border.format_bottom(render_width));
        result.push_str("\n");

        result
    }

    /// Measures the width of a table.
    /// 
    /// Column breaks are used to constrain the render width of columns and 
    ///  are considered along with the content of the header cells.
    ///
    /// # Arguments
    ///
    /// * `self` - The table being measured.
    fn measure_width(
        self: &Table
    ) -> usize {
        let mut header_width = 0;

        // Iterate through the header row
        let mut column_break_ix = 0;
        let content_break = ColumnBreak { width: BreakWidth::Content };
        for cell in self.column_headers.iter() {
            // Get the next column break (if one is available)
            let column_break: &ColumnBreak = 
                if column_break_ix < self.column_breaks.len() {
                    &self.column_breaks[column_break_ix]
                } else {
                    // Use content-width break for additional columns
                    &content_break
                };
            // Calculate the width of this header cell
            header_width += cell.measure_width(column_break);
            // Increment column index
            column_break_ix += 1;
        }

        // Add space for the outer borders
        header_width += 2;

        // Add space for vertical splits separators between columns
        header_width += self.column_headers.len() - 1;

        header_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use crate::content::{Content, Alignment, Wrap};
    use crate::data_item::DataItem;
    use crate::cell;

    #[test]
    fn measure_header_one_column() {
        let breaks = vec!(
            ColumnBreak { width: BreakWidth::Fixed(15) }
        );

        let col_headers = TableRow::from(
            vec!(
                cell!("{^}", "test", "hello")
            )
        );

        let mut table = Table::from(
            breaks,
            col_headers,
            Vec::new(),
            Vec::new()
        );

        // Expect 15 chars for column, 2 for outer border chars
        let expected_width = 17;

        assert_eq!(table.measure_width(), expected_width);
    }

    // #[test]
    // fn measure_header_two_columns() {
    //     let breaks = vec!(
    //         ColumnBreak { width: BreakWidth::Fixed(15) },
    //         ColumnBreak { width: BreakWidth::Fixed(15) }
    //     );

    //     let col_headers = TableRow::from(
    //         vec!(
    //             TableCell::from_data_item(
    //                 DataItem::from(
    //                     vec!(
    //                         Content::new(
    //                             String::from("test"),
    //                             Color::White,
    //                             Alignment::Center,
    //                             Wrap::NoWrap
    //                         )
    //                     )
    //                 )
    //             ),
    //             TableCell::from_data_item(
    //                 DataItem::from(
    //                     vec!(
    //                         Content::new(
    //                             String::from("test"),
    //                             Color::White,
    //                             Alignment::Center,
    //                             Wrap::NoWrap
    //                         )
    //                     )
    //                 )
    //             )
    //         )
    //     );

    //     let mut table = Table::from(
    //         breaks,
    //         col_headers,
    //         Vec::new(),
    //         Vec::new()
    //     );
        
    //     // Expect 33 chars. 2 x 15 columns + 2 for outer border, 1 for split
    //     let expected_width = 33;

    //     assert_eq!(table.measure_width(), expected_width);
    // }
}
