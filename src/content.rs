use colored::Color;

/// Describes how content should be aligned.
pub enum Alignment {
    Left,
    Center,
    Right
}

/// Describes whether content will wrap or truncate.
pub enum Wrap {
    /// Content will be truncated when over-width
    NoWrap,
    // Content will wrap when over-width
    Wrap
}

/// Represents a line of content.
pub struct Content {
    content: String,
    color: Color,
    alignment: Alignment,
    wrap: Wrap 
}

impl Content {
    /// Returns a new content item.
    /// 
    /// # Arguments
    /// 
    /// * `content` - The string content.
    /// * `color` - The content color.
    /// * `alignment` - The content aligment.
    /// * `wrap` - The content wrapping method.
    pub fn new(
        content: String,
        color: Color, 
        alignment: Alignment, 
        wrap: Wrap
    ) -> Content {
        Content {
            content,
            color,
            alignment,
            wrap
        }
    } 

    /// Formats a content line.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content to format.
    /// * `width` - The width at which to wrap or truncate.
    pub fn format(
        self: &Content,
        width: usize
    ) -> String {
        let content_len = self.content.len();
        // Pad or truncate line as needed
        if width > content_len {
            // Too short: Right-pad
            format!("{}{}",
                self.content,
                (0..(width - content_len))
                    .map(|_| " ")
                    .collect::<String>()
            )
        } else if width == content_len {
            // Exact size
            self.content.to_string()
        } else {
            // Too long: Wrap or truncate
            self.wrap_or_truncate(width)
        }
    }

    /// Measures the width of content.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content to measure.
    pub fn measure_width(
        self: &Content
    ) -> usize {
        self.content.len()
    }

    /// Measures the height of this content if formatted to a specific width.
    /// 
    /// This is useful for determining if content will use additional height 
    ///  when wrapped.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content being measured.
    /// * `width` - The width the content is being measured into.
    pub fn measure_height(
        self: &Content,
        width: usize
    ) -> usize {
        let content_len = self.content.len();

        // Calculate number of whole lines needed
        let mut height = content_len.div_euclid(width);
        
        // Check if there is a final partial line
        let partial_line_len = content_len.rem_euclid(width); 
        if partial_line_len != 0 {
            height += 1;
        }

        height
    }

    fn wrap_or_truncate(
        self: &Content, 
        width: usize
    ) -> String {
        let content_len = self.content.len();

        match self.wrap {
            // Truncate on single line
            Wrap::NoWrap => {
                format!("{}...",
                    self.content[0..(width - 3 )].to_string(),
                )
            }
            // Wrap to multiple lines
            Wrap::Wrap => {
                let num_lines = self.measure_height(width);
                let partial_line_len = content_len.rem_euclid(width);
                // Collect the wrapped lined
                let mut parts: Vec<&str> = Vec::new();
                for line_ix in 0..num_lines {
                    let from = line_ix * width;
                    let to = 
                        if line_ix < num_lines { from + width }
                        else { from + partial_line_len };
                    parts.push(&self.content[from..to])
                }
                // Join the lines with newlines
                parts.join("\n")
            }
        }
    }
}