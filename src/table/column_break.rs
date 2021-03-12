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
macro_rules! brk {
    ( c ) => {
        ColumnBreak::Content
    };

    ( {$t:literal:$w:literal} ) => {
        match $t {
            'f' => ColumnBreak::Fixed($w),
            'm' => ColumnBreak::Minimum($w)
        }
    }
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
        let t = &format[1..1];
        let w = &format[3..3];

        Ok(ColumnBreak::Fixed(15))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
