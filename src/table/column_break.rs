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

impl FromStr for ColumnBreak {
    type Err = std::string::ParseError;

    /// Returns a column break from a string.
    /// 
    /// # Arguments
    /// 
    /// * `format` - The format string describing the break.
    /// 
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        let content = format[1..format.len()-1].split(':').collect::<Vec<&str>>();

        let t = content[0];
        let w = usize::from_str(content[1]).unwrap();

        match t {
            "f" => Ok(ColumnBreak::Fixed(w)),
            "m" => Ok(ColumnBreak::Minimum(w)),
            "c" | _ => Ok(ColumnBreak::Content)
        }

        // Ok(ColumnBreak::Fixed(15))
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
}
