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

pub struct ContentIterator {
    parts: Vec<String>,
    next_part_ix: usize
}

impl Iterator for ContentIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.next_part_ix < self.parts.len() {
            Some(self.parts[self.next_part_ix].to_string())
        } else {
            None
        }
    }
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

    /// Returns an iterator for the line parts of a content.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content to iterate.
    /// * `width` - The width at which to wrap or truncate.
    pub fn get_iterator(
        self: &Content,
        width: usize
    ) -> ContentIterator {
        let content_len = self.content.len();

        let mut result: Vec<String> = Vec::new();

        match self.wrap {
            // Truncate on single line
            Wrap::NoWrap => {
                result.push(
                    format!("{}...",
                        self.content[0..(width - 3 )].to_string(),
                    )
                );
            }
            // Wrap to multiple lines
            Wrap::Wrap => {
                let num_lines = self.measure_height(width);
                let partial_line_len = content_len.rem_euclid(width);
                
                // Collect the line parts
                for line_ix in 0..num_lines {
                    let from = line_ix * width;
                    let to = 
                        if line_ix < num_lines - 1 { from + width }
                        else { from + partial_line_len };
                    result.push(
                        Content::pad(
                            &self.content[from..to], 
                            &self.alignment, 
                            width)
                        )
                }
            }
        }

        ContentIterator {
            parts: result,
            next_part_ix: 0
        }
    }

    /// Formats part of a content line.
    /// 
    /// This considers alignment and wrapping.
    /// 
    /// If the content does not wrap (i.e. truncates) then there is only one 
    ///  part, the truncated line. However, if the content does wrap then 
    ///  the requested line part is returned.
    /// 
    /// Wrapping breaks are determined based upon the supplied width.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content to format.
    /// * `part_ix` - The index of the line part to format.
    /// * `width` - The width at which to wrap or truncate.
    // fn format_part(
    //     self: &Content,
    //     part_ix: usize,
    //     width: usize
    // ) -> String {
    //     // Truncated lines 
    //     if !self.will_wrap() {
    //         Content::pad(&self.content, self.alignment, width)
    //     }
    //     else {
    //         // Get the requested line part
    //         let content_len = self.content.len();

    //         let from = part_ix * width;
    //         let to = 
    //             if (from + width) > content_len { content_len - 1}
    //             else { from + width }; 
            
    //         Content::pad(&self.content[from..to], self.alignment, width)
    //     }
    // }

    fn pad(
        line: &str,
        alignment: &Alignment,
        width: usize
    ) -> String {
        let content_len = line.len();

        if content_len == width {
            line.to_string()
        } else {
            let padding = width - content_len;

            match alignment {
                Alignment::Left => {
                    format!("{}{}",
                        line,    
                        (0..padding)
                                .map(|_| " ")
                                .collect::<String>(),        
                    )
                }
                Alignment::Center => {
                    let left_pad = padding / 2;
                    let right_pad = 
                        if padding.rem_euclid(2) == 0 { padding / 2 }
                        else { (padding / 2) + 1 };
                    format!("{}{}{}",
                        (0..left_pad)
                            .map(|_| " ")
                            .collect::<String>(),
                        line,
                        (0..right_pad)
                            .map(|_| " ")
                            .collect::<String>(),
                    )
                }
                Alignment::Right => {
                    format!("{}{}",
                        (0..padding)
                            .map(|_| " ")
                            .collect::<String>(),
                        line,
                    )
                }
            }
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
        // Calculate height if content will wrap
        if self.will_wrap() {
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
        else {
            // Content will be truncated (always height 1)
            1
        }
    }

    /// Returns a flag indicating whether this content will wrap.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The content being measured.
    pub fn will_wrap(
        self: &Content
    ) -> bool {
        match self.wrap {
            Wrap::Wrap => true,
            Wrap::NoWrap => false
        }
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
                // Collect the wrapped lines
                let mut parts: Vec<&str> = Vec::new();

                for line_ix in 0..num_lines {
                    let from = line_ix * width;
                    let to = 
                        if line_ix < num_lines - 1 { from + width }
                        else { from + partial_line_len };
                    parts.push(&self.content[from..to])
                }
                // Join the lines with newlines
                parts.join("\n")
            }
        }
    }
}