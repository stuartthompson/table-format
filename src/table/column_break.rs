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
        let content = 
            s[1..s.len()-1].split(':').collect::<Vec<&str>>();
        
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_fixed() {
        let cb = ColumnBreak::from_str("{f:15}").unwrap();
        assert_eq!(
            format!("{:?}", cb), 
            format!("{:?}", ColumnBreak::Fixed(15))
        );
    }

    #[test]
    fn from_str_min_width() {
        let cb = ColumnBreak::from_str("{m:15}").unwrap();
        assert_eq!(
            format!("{:?}", cb), 
            format!("{:?}", ColumnBreak::Minimum(15))
        );
    }

    #[test]
    fn from_str_content() {
        let cb = ColumnBreak::from_str("{c}").unwrap();
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
}
