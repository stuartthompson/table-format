use std::fmt::Display;
use colored::Color;

/// Describes how content should be aligned.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right
}

/// Describes whether content will wrap or truncate.
#[derive(Debug)]
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

macro_rules! content {
    ($c: ident, $style:tt, $content:expr) => {
        $c.add_lines(stringify!($style), $content);
    };
    ($c: ident, $style:tt, $($content_rest:expr),*) => {
        content!($c, $style, vec!($($content_rest),*));
    };
}

/// Represents a line of content.
#[derive(Debug)]
pub struct Content {
    content: String,
    color: Color,
    alignment: Alignment,
    wrap: Wrap,
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

    pub fn add_content_line(
        line: &str
    ) {
        println!("Content: {}", line);
    }

    // pub fn add_content_line(
    //     style: &str, 
    //     line: &str
    // ) {
    //     println!("Adding line: Style: {}, Line: {}", style, line);
    // }

    pub fn from_form1(
        style: &str,
        content: &str,
    ) -> Content {
        println!("Content: {}", content);
        println!("Style: {}", style);

        Content::new(
            format!("{} {}", content, style),
            Color::Red,
            Alignment::Left,
            Wrap::Wrap
        )
    }

    pub fn from_tokens(
        content: &str,
        tokens: &str
    ) -> Content {

        // Extract color
        let color = match tokens[1..tokens.len()-1].find(':') {
            Some(colon_index) => {
                let color_token = &tokens[1..colon_index].trim();
                match *color_token {
                    "Red" => { Color::Red },
                    "Blue" => { Color::Blue },
                    _ => { Color::White }
                }
            },
            None => {
                Color::White
            }
        };

        Content::new( 
            format!("Content: {}", content),
            color,
            Alignment::Left,
            Wrap::Wrap
        )
    }

    pub fn from_string(
        content: String
    ) -> Content {
        Content {
            content,
            color: Color::White,
            alignment: Alignment::Left,
            wrap: Wrap::NoWrap
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
                // Calculate any overage
                if content_len <= width {
                    result.push(
                        Content::pad(
                            &self.content,
                            &self.alignment, 
                            width
                        )
                    );
                } else {
                    result.push(
                        format!("{}...",
                            self.content[0..(width - 3)].to_string(),
                        )
                    );
                }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_macro_left() {
        let content = content!(L "testing".to_string());

        assert_eq!(content.alignment, Alignment::Left);
    }

    #[test]
    fn test_tcon_align_left() {
        let content = tcon!({Red:<W}, "testing");

        println!("{:?}", content);

        assert_eq!(content.alignment, Alignment::Left);
    }

    #[test]
    fn test_tcon_form1() {
        let content = tcon2!("{r:<;W} {b:^} {w:^;w}", "testing", "hello", "a");

        //println!("{:?}", content);

        //assert_eq!(content.alignment, Alignment::Left);
    }

    #[test]
    fn test_measure_width() {
        let content = Content::from_string("testing".to_string());

        assert_eq!(7, content.measure_width());
    }

    #[test]
    fn test_pad_left_aligned() {
        let padded = Content::pad("testing", &Alignment::Left, 10);

        assert_eq!(padded, "testing   ");
    }

}