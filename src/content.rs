mod content_style;

use std::fmt::Display;
use colored::{Color, Colorize};
pub use content_style::{ContentStyle, Alignment, Wrap};

pub struct ContentIterator {
    parts: Vec<String>,
    next_part_ix: usize
}

impl Iterator for ContentIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.next_part_ix < self.parts.len() {
            // Get the next line part
            let line_part = self.parts[self.next_part_ix].to_string();
            
            // Increment the line part counter
            self.next_part_ix += 1;

            // Return the line part
            Some(line_part)
        } else {
            None
        }
    }
}

/// Represents a line of content.
#[derive(Debug, Clone)]
pub struct Content {
    content: String,
    pub style: ContentStyle,
}

impl std::str::FromStr for Content {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Content { content: s.to_string(), style: ContentStyle::default() })
    } 
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
        style: ContentStyle,
    ) -> Content {
        Content {
            content,
            style
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

        match self.style.wrap {
            // Truncate on single line
            Wrap::Truncate => {
                // Pad or truncate
                if content_len <= width {
                    result.push(
                        Content::format(
                            &self.content,
                            &self.style, 
                            width
                        )
                    );
                } else {
                    result.push(
                        Content::format(
                            &self.content[0..(width - 3)],
                            &self.style,
                            width
                        )
                    );
                }
            },
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
                        Content::format(
                            &self.content[from..to], 
                            &self.style, 
                            width)
                    );
                }
            },
        }

        ContentIterator {
            parts: result,
            next_part_ix: 0
        }
    }

    fn format(
        line: &str,
        style: &ContentStyle,
        width: usize
    ) -> String {
        let mut result = Content::pad(line, &style.alignment, width);

        // Apply colors
        if style.fg_color != None {
            result = result.color(style.fg_color.unwrap()).to_string();
        }
        if style.bg_color != None {
            result = result.on_color(style.bg_color.unwrap()).to_string();
        }

        result   
    }

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
        match self.style.wrap {
            Wrap::Wrap => true,
            Wrap::Truncate => false
        }
    }
}

#[cfg(test)]
mod tests {

}
