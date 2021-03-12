#![recursion_limit="256"]


mod content;
mod data_item;
mod data_source;
mod table_data_source;
mod vec_data_source;
mod table;

use table::Table;
use data_item::DataItem;
// use data_source::WSDataFrame;
use vec_data_source::VecDataSource;

#[cfg(test)]
mod tests {
    use super::*;
    use table::column_break::ColumnBreak;
    use table::table_row::TableRow;
    use table::table_cell::TableCell;

    #[test]
    fn test_simple_vector_table() {
        // let table = table!(
        //     breaks!(b!(F(15)), b!(F(10))), 
        //     row!("{c^}", "Food", "Count"), 
        //     "Fish", "3", "Pears", "5", "Pizza", "13"
        // );

        // let output = table.format();

        // //let expected = String::from("+--------------------------+\n|     Food      |  Count   |\n+--------------------------+\n|FishFishFishFis|3         |\n|hFishFish      |          |\n+--------------------------+\n|Apples         |5         |\n+--------------------------+\n|Pizza          |13        |\n+--------------------------+\n");

        // println!("1-------10--------20--------30--------40--------50--------60--------70--------80");
        // println!("''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|");
        // println!("{}", output);

        //assert_eq!(expected, output);
    }
}