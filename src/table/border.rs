use colored::Color;

#[derive(Debug)]
pub struct Border {
    pub top_left: char,
    pub top: char,
    pub top_right: char,
    pub top_split: char,
    pub bottom_left: char,
    pub bottom: char,
    pub bottom_right: char,
    pub bottom_split: char,
    pub left: char,
    pub left_split: char,
    pub right: char,
    pub right_split: char,
    pub vertical_split: char,
    pub vertical_split_intersect_left: char,
    pub vertical_split_intersect_right: char,
    pub vertical_split_intersect_both: char,
    pub horizontal_split: char,
    pub horizontal_split_intersect_top: char,
    pub horizontal_split_intersect_bottom: char,
    pub horizontal_split_intersect_both: char,
    pub color: Color
}

impl Border {
    #[must_use]
    pub fn default() -> Border {
        Border {
            top_left: '+',
            top: '-',
            top_right: '+',
            top_split: '+',
            bottom_left: '+',
            bottom: '-',
            bottom_right: '+',
            bottom_split: '+',
            left: '|',
            left_split: '+',
            right: '|',
            right_split: '+',
            vertical_split: '|',
            vertical_split_intersect_left: '+',
            vertical_split_intersect_right: '+',
            vertical_split_intersect_both: '+',
            horizontal_split: '-',
            horizontal_split_intersect_top: '+',
            horizontal_split_intersect_bottom: '+',
            horizontal_split_intersect_both: '+',
            color: Color::Cyan
        }
    }

    /// Formats the top border
    #[must_use]
    pub fn format_top(
        self: &Border,
        widths: &[usize]
    ) -> String {
        let mut result: String = String::from(self.top_left);
        for ix in 0..widths.len() {
            result.push_str(
              &(0..widths[ix])
                  .map(|_| self.top)
                  .collect::<String>()
            );
            if ix < widths.len() - 1 {
               result.push_str(String::from(self.top_split).as_str());
            }
        }
        result.push_str(String::from(self.top_right).as_str());
        result
    }

    /// Formats the bottom border
    #[must_use]
    pub fn format_bottom(
        self: &Border,
        widths: &[usize]
    ) -> String {
        let mut result: String = String::from(self.bottom_left);
        for ix in 0..widths.len() {
            result.push_str(
                &(0..widths[ix])
                    .map(|_| self.bottom)
                    .collect::<String>()
            );
            if ix < widths.len() - 1 {
                result.push_str(String::from(self.bottom_split).as_str());
            }
        }
        result.push_str(String::from(self.bottom_right).as_str());
        result
    }

    /// Formats the left border.
    #[must_use]
    pub fn format_left(
        self: &Border
    ) -> String {
        format!("{}", self.left)
    }

    /// Formats the right border.
    #[must_use]
    pub fn format_right(
        self: &Border
    ) -> String {
        format!("{}", self.right)
    }

    /// Formats a horizontal split
    #[must_use]
    pub fn format_horizontal_split(
        self: &Border,
        widths: &[usize]
    ) -> String {
        let mut result: String = String::from(self.left_split);
        for ix in 0..widths.len() {
            result.push_str(
                &(0..widths[ix])
                    .map(|_| self.horizontal_split)
                    .collect::<String>()
            );
            if ix < widths.len() - 1 {
                result.push_str(
                    String::from(self.horizontal_split_intersect_both).as_str()
                );
            }
        }
        result.push_str(String::from(self.right_split).as_str());
        result
    }

    pub fn format_vertical_split(
        self: &Border
    ) -> String {
        format!("{}", self.vertical_split)
    }
}
