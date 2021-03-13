use colored::Color;

#[derive(Debug)]
pub struct Border {
    top_left: char,
    top: char,
    top_right: char,
    top_split: char,
    bottom_left: char,
    bottom: char,
    bottom_right: char,
    bottom_split: char,
    left: char,
    left_split: char,
    right: char,
    right_split: char,
    vertical_split: char,
    vertical_split_intersect_left: char,
    vertical_split_intersect_right: char,
    vertical_split_intersect_both: char,
    horizontal_split: char,
    horizontal_split_intersect_left: char,
    horizontal_split_intersect_right: char,
    horizontal_split_intersect_both: char,
    color: Color
}

impl Border {
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
            horizontal_split_intersect_left: '+',
            horizontal_split_intersect_right: '+',
            horizontal_split_intersect_both: '+',
            color: Color::Cyan
        }
    }

    /// Formats the top border
    pub fn format_top(
        self: &Border,
        width: usize
    ) -> String {
        format!("{}{}{}", 
            self.top_left,
            &(0..width - 2)
                .map(|_| self.top)
                .collect::<String>(),
            self.top_right)
    }

    /// Formats the bottom border
    pub fn format_bottom(
        self: &Border,
        width: usize
    ) -> String {
        format!("{}{}{}", 
            self.bottom_left,
            &(0..width - 2)
                .map(|_| self.bottom)
                .collect::<String>(),
            self.bottom_right)
    }

    pub fn format_left(
        self: &Border
    ) -> String {
        format!("{}", self.left)
    }

    pub fn format_right(
        self: &Border
    ) -> String {
        format!("{}", self.right)
    }

    /// Formats a horizontal split
    pub fn format_horizontal_split(
        self: &Border,
        width: usize
    ) -> String {
        format!("{}{}{}",
            self.left_split,
            &(0..width - 2)
                .map(|_| self.horizontal_split)
                .collect::<String>(),
            self.right_split
        )
    }

    pub fn format_vertical_split(
        self: &Border
    ) -> String {
        format!("{}", self.vertical_split)
    }
}