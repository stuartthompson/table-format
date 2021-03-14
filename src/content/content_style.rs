use std::str::FromStr;
use colored::{Color, Colorize};

#[derive(Debug, Clone)]
pub enum CellWidth {
    // The cell width is fixed
    Fixed(usize),
    // The cell is always at least a minimum width
    Minimum(usize),
    // The cell takes on the width of its content
    Content,
}

impl CellWidth {
    pub fn default() -> CellWidth {
        CellWidth::Content
    }
}

/// Describes how content should be aligned.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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






#[allow(unused_macros)]
#[macro_export]
macro_rules! content_style {
    ( $style:literal ) => {
        ContentStyle::from_format($style)
    }    
}

/// Represents the style to apply to a line of content.
#[derive(Debug, Clone)]
pub struct ContentStyle {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub alignment: Alignment,
    pub wrap: Wrap,
    pub width: CellWidth
}

impl ContentStyle {
    pub fn default() -> ContentStyle {
        ContentStyle {
            fg_color: None,
            bg_color: None,
            alignment: Alignment::Left,
            wrap: Wrap::Truncate,
            width: CellWidth::Content,
        }
    }

    pub fn new(
        fg_color: Option<Color>,
        bg_color: Option<Color>,
        alignment: Alignment,
        wrap: Wrap,
        width: CellWidth
    ) -> ContentStyle {
        ContentStyle {
            fg_color,
            bg_color,
            alignment,
            wrap,
            width,
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

            // Width specifier (consumes until matching token)
            if token == ':' {
                // TODO: Clean up this logic (should be a common width fn)
                match format[token_ix+1..tokens.len()+1].find(':') {
                    Some(ix) => {
                        let width = format[token_ix+1..token_ix+ix+1].parse::<usize>().unwrap();
                        style.width = CellWidth::Fixed(width);
                        token_ix += ix + 1;
                    }
                    None => { /* Ignore (no matching token) */ }
                }
            }

            // Width specifier (consumes until matching token)
            if token == '|' {
                // TODO: Clean up this logic (should be a common width fn)
                match format[token_ix+1..tokens.len()+1].find('|') {
                    Some(ix) => {
                        let width = format[token_ix+1..token_ix+ix+1].parse::<usize>().unwrap();
                        style.width = CellWidth::Minimum(width);
                        token_ix += ix + 1;
                    }
                    None => { /* Ignore (no matching token) */ }
                }
            }
        }

        style
    }

    // fn parse_cell_width(
    //     tt: Vec<char>,
    //     t_ix: &mut usize
    // ) -> Option<CellWidth> {
    //     match tt[*t_ix] {
    //         ':' | '|' => {

    //         }
    //         _ => None
    //     }


    //     if let Some(n) = s.find(token) {
    //         let width = s[1..n].parse::<usize>().unwrap();
    //     } else {
    //         Result<None, 0>
    //     }
    //     match  {
    //         Some(ix) => { 
    //             style.width = CellWidth::Fixed(width);
    //             token_ix += ix + 1;
    //         }
    //         None => { /* Ignore (no matching token) */ }
    //     }
    // }

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

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;

   #[test]
   fn from_format_fixed_width() {
        let style = ContentStyle::from_format("{c^;:15:}");
   
        let expected = 
            ContentStyle {
                fg_color: Some(Color::Cyan),
                bg_color: None,
                alignment: Alignment::Center,
                wrap: Wrap::Wrap,
                width: CellWidth::Fixed(15)
            };

        assert_eq!(
            format!("{:?}", style),
            format!("{:?}", expected)
        );
    }

}