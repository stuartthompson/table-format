pub struct Line {
    content: String
}

impl Line {
    pub fn from(
        content: String
    ) -> Line {
        Line {
            content
        }
    }

    /// Formats the content of a line to a specific width.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The line to format.
    /// * `width` - The desired width of the formatted content.
    pub fn format(
        self: &Line, 
        width: u8
    ) -> String {
        if width > self.content.len() as u8 {
            // Right pad: Content is too short
            format!("{}{}",
                self.content,
                (0..(width - self.content.len() as u8))
                    .map(|_| " ")
                    .collect::<String>()
            )
        } else if width == self.content.len() as u8 {
            // Content is exactly the requested size
            self.content.to_string()
        } else {
            // Truncate: Content is too long
            format!("{}...",
                self.content[0..(self.content.len() - 3)].to_string(),
            )
        }
    }

    /// Measures the width of a line.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The line to measure.
    pub fn measure_width(
        self: &Line
    ) -> u8 {
        self.content.len() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_width_of_single_line() {
        let line = Line::from(String::from("test"));

        assert_eq!(line.measure_width(), 4);
    }

    #[test]
    fn format_right_pad() {
        let line = Line::from(String::from("test"));

        let formatted = line.format(10);

        assert_eq!(format!("{}", formatted), "test      ");
    }
}