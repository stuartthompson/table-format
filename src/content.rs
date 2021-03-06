use std::fmt::Display;
use colored::{Color, Colorize};

/// Describes how content should be aligned.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right
}

impl Alignment {
    fn from_token(token: char) -> Option<Alignment> {
        match token {
            '<' => Some(Alignment::Left),
            '^' => Some(Alignment::Center),
            '>' => Some(Alignment::Right),
            _ => None
        }
    }
}

/// Describes whether content will wrap or truncate.
#[derive(Debug)]
pub enum Wrap {
    /// Content will be truncated when over-width
    Truncate,
    // Content will wrap when over-width
    Wrap
}

impl Wrap {
    fn from_token(token: char) -> Option<Wrap> {
        match token {
            ';' => Some(Wrap::Wrap),
            '.' => Some(Wrap::Truncate),
            _ => None
        }
    }
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

/// Represents the style to apply to a line of content.
#[derive(Debug)]
pub struct ContentStyle {
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    alignment: Alignment,
    wrap: Wrap,
}

impl ContentStyle {
    pub fn default() -> ContentStyle {
        ContentStyle {
            fg_color: None,
            bg_color: None,
            alignment: Alignment::Left,
            wrap: Wrap::Truncate
        }
    }

    pub fn new(
        fg_color: Option<Color>,
        bg_color: Option<Color>,
        alignment: Alignment,
        wrap: Wrap
    ) -> ContentStyle {
        ContentStyle {
            fg_color,
            bg_color,
            alignment,
            wrap,
        }
    }

    pub fn from_format(format: &str) -> ContentStyle {
        // Start with defaults
        let mut style = ContentStyle::default();

        // Iterate tokens
        let tokens: Vec<char> = format[1..format.len() - 1].chars().collect();
        let mut token_ix = 0;
        while token_ix < tokens.len() {
            let token = tokens[token_ix];
           
            // Foreground color
            match ContentStyle::color_from_token(token) {
                Some(color) => style.fg_color = Some(color),
                None => {}
            }
            // Alignment
            match Alignment::from_token(token) {
                Some(alignment) => style.alignment = alignment,
                None => {}
            }
            // Wrap
            match Wrap::from_token(token) {
                Some(wrap) => style.wrap = wrap,
                None => {}
            }

            // Background color (consumes two tokens)
            if token == '-' {
                // Avoid problem if - is last token (with no color code)
                if tokens.len() > token_ix + 1 {
                    style.bg_color = 
                        ContentStyle::color_from_token(tokens[token_ix + 1]);
                    // Consume next token (to skip the background color code)
                    token_ix += 1;
                }
            }
            token_ix += 1;
        }

        println!("Style is: {:?}", style);

        style
    }

    fn color_from_token(
        token: char
    ) -> Option<Color> {
        match token {
            'w' => Some(Color::White),
            'l' => Some(Color::Black),
            'r' => Some(Color::Red),
            'g' => Some(Color::Green),
            'y' => Some(Color::Yellow),
            'b' => Some(Color::Blue),
            'm' => Some(Color::Magenta),
            'c' => Some(Color::Cyan),
            'W' => Some(Color::BrightWhite),
            'L' => Some(Color::BrightBlack),
            'R' => Some(Color::BrightRed),
            'G' => Some(Color::BrightGreen),
            'Y' => Some(Color::BrightYellow),
            'B' => Some(Color::BrightBlue),
            'M' => Some(Color::BrightMagenta),
            'C' => Some(Color::BrightCyan),
            _ => None,
        }
    }
}

/// Represents a line of content.
#[derive(Debug)]
pub struct Content {
    content: String,
    style: ContentStyle,
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

    pub fn from_string(
        content: String
    ) -> Content {
        Content {
            content,
            style: ContentStyle::default(),
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
    use super::*;

    // #[test]
    // fn test_content_macro() {
    //     let content = c2!("{w<+}", "testing");

    //     assert_eq!(content.alignment, Alignment::Left);
    // }

    // #[test]
    // fn test_tcon_align_left() {
    //     let content = tcon!({Red:<W}, "testing");

    //     println!("{:?}", content);

    //     assert_eq!(content.alignment, Alignment::Left);
    // }

    // #[test]
    // fn test_tcon_form1() {
    //     let content = tcon2!("{r:<;W} {b:^} {w:^;w}", "testing", "hello", "a");

    //     //println!("{:?}", content);

    //     //assert_eq!(content.alignment, Alignment::Left);
    // }

    // #[test]
    // fn test_measure_width() {
    //     let content = Content::from_string("testing".to_string());

    //     assert_eq!(7, content.measure_width());
    // }

    // #[test]
    // fn test_pad_left_aligned() {
    //     let padded = Content::pad("testing", &Alignment::Left, 10);

    //     assert_eq!(padded, "testing   ");
    // }

}