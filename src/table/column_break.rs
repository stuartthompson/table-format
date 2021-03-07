#[derive(Debug)]
pub enum BreakWidth {
    // The column width is fixed
    Fixed(usize),
    // The column is always at least a minimum width
    Minimum(usize),
    // The column takes on the width of its header content
    Content,
}

/// Column breaks describe where content should be divided
#[derive(Debug)]
pub struct ColumnBreak {
    pub width: BreakWidth
}


// breaks!(f5, m5, c)

#[allow(unused_macros)]
#[macro_export]
macro_rules! b {
    (F($w:tt)) => {
        BreakWidth::Fixed($w)
    };
    (M($w:tt)) => {
        BreakWidth::Minimum($w)
    };
    (C) => {
        BreakWidth::Content
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! breaks {
    ($( $b:tt ),*) => {{
        let mut v = Vec::new();
        $( v.push(ColumnBreak { width: b!($b) }); )*
        v
    }};
} 

#[allow(unused_macros)]
#[macro_export]
macro_rules! bks {
    ($(F($f:tt)),*, $(M($m:tt)),*, $(C),*) => {{
        let mut v = Vec::new();
        $( 
            v.push(ColumnBreak { width: BreakWidth::Fixed($f) }); 
            v.push(ColumnBreak { width: BreakWidth::Minimum($m) });
            v.push(ColumnBreak { width: BreakWidth::Content });
        )*
        v
    }};
}

    // ($( $style:expr => $content:expr ),*) => {
    //     {
    //         let mut tr = TableRow::new();
    //         $( tr.add_cell(crate::cell!($style, $content)); )*;
    //         tr
    //     }
    // };
    // ($style:expr, $($content:expr ),*) => {
    //     {
    //         let mut tr = TableRow::new();
    //         $( tr.add_cell(crate::cell!($style, $content)); )*
    //         tr
    //     }
    // };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaks_macro() {
        let b1 = b!(F(15));
        let b2 = b!(M(25));
        let b3 = b!(C);

        let b = bks!(F(15), F(8), M(25), M(3), C, C);
        assert_eq!(
            format!("{:?}", b),
            format!("{:?}", 
                vec!(
                    ColumnBreak { width: BreakWidth::Fixed(15)},
                    ColumnBreak { width: BreakWidth::Minimum(25)},
                    ColumnBreak { width: BreakWidth::Content},
                )    
            )
        );
    }
}