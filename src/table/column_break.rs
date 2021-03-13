use std::str::FromStr;

#[derive(Debug)]
pub enum ColumnBreak {
    // The column width is fixed
    Fixed(usize),
    // The column is always at least a minimum width
    Minimum(usize),
    // The column takes on the width of its header content
    Content,
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! breaks {
    ( $($break:expr),* ) => {{
        let mut breaks = Vec::new();
        $(breaks.push(ColumnBreak::from_string($break));)*
        breaks
    }}
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String) 
}

impl FromStr for ColumnBreak {
    type Err = ParseError;

    /// Returns a column break from a string.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format string describing the break.
   fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().len() == 0 {
            return Err(ParseError::InvalidFormat(
                String::from("format cannot be empty")));
        }

        // Split contents of format string into type and params
        let content = s.split(':').collect::<Vec<&str>>();
        
        match content[0] {
            "f" => {
                let width = usize::from_str(content[1]).unwrap();
                Ok(ColumnBreak::Fixed(width))
            },
            "m" => {
                let width = usize::from_str(content[1]).unwrap();
                Ok(ColumnBreak::Minimum(width))
            },
            "c" | _ => Ok(ColumnBreak::Content)
        }
    }
}

impl ColumnBreak {
    pub fn from_string(s: &str) -> ColumnBreak {
        ColumnBreak::from_str(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_fixed() {
        let cb = ColumnBreak::from_str("f:15").unwrap();
        assert_eq!(
            format!("{:?}", cb), 
            format!("{:?}", ColumnBreak::Fixed(15))
        );
    }

    #[test]
    fn from_str_min_width() {
        let cb = ColumnBreak::from_str("m:15").unwrap();
        assert_eq!(
            format!("{:?}", cb), 
            format!("{:?}", ColumnBreak::Minimum(15))
        );
    }

    #[test]
    fn from_str_content() {
        let cb = ColumnBreak::from_str("c").unwrap();
        assert_eq!(
            format!("{:?}", cb),
            format!("{:?}", ColumnBreak::Content)
        );
    }

    /// Tests that 
    #[test]
    #[should_panic] 
    fn from_str_invalid_empty() {
        ColumnBreak::from_str("").unwrap();
    }

    #[test]
    fn from_breaks_macro() {
        let breaks = breaks!("f:15", "f:10");

        let expected = vec!(
            ColumnBreak::Fixed(15), 
            ColumnBreak::Fixed(10)
        );

        assert_eq!(
            format!("{:?}", breaks),
            format!("{:?}", expected)
        );
    }
}
