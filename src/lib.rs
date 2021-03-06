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
    use colored::Color;
    use content::{Content, Alignment, Wrap};
    use table::column_break::{ColumnBreak, BreakWidth};
    use table::table_row::TableRow;
    use table::table_cell::TableCell;

    #[test]
    fn test_simple_vector_table() {
        let breaks = vec!(
            ColumnBreak { width: BreakWidth::Fixed(15) },
            ColumnBreak { width: BreakWidth::Fixed(10) }
        );

        let col_headers = TableRow::from(
            vec!(
                cell!("{r>}", "Food", "t"),
                cell!("{b^}", "Count", "t"),
            )
        );

        let data = VecDataSource::from(
            vec!("FishFishFishFishFishFish", "3", "Apples", "5", "Pizza", "13"));

        let table = Table::from_vec_data_source(
            breaks,
            col_headers,
            Vec::new(),
            data
        );

        let output = table.format();

        let expected = String::from("+--------------------------+\n|     Food      |  Count   |\n+--------------------------+\n|FishFishFishFis|3         |\n|hFishFish      |          |\n+--------------------------+\n|Apples         |5         |\n+--------------------------+\n|Pizza          |13        |\n+--------------------------+\n");

        println!("1-------10--------20--------30--------40--------50--------60--------70--------80");
        println!("''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|");
        println!("{}", output);

        assert_eq!(expected, output);
    }

    // #[test]
    // fn test_multiline_data_source_table() {
    //     let mut data = WSDataFrame::from(String::from("asdfasds"));

    //     let byte1_header = Content::new(
    //         String::from("Byte 1"),
    //         Color::White,
    //         Alignment::Center,
    //         Wrap::NoWrap
    //     );
    //     let byte2_header = Content::new(
    //         String::from("Byte 2"),
    //         Color::White,
    //         Alignment::Center,
    //         Wrap::NoWrap
    //     );
    //     let byte3_header = Content::new(
    //         String::from("Byte 3"),
    //         Color::White,
    //         Alignment::Center,
    //         Wrap::NoWrap
    //     );

    //     let columns = vec!(
    //         TableColumn::fixed(byte1_header, 15),
    //         TableColumn::fixed(byte2_header, 15),
    //         TableColumn::fixed(byte3_header, 15),
    //     );
    //     let table = Table::from(&mut data, columns);

    //     let output = table.format(80);

    //     let expected = String::from("+-------------------------------+\n|Food           |Count          |\n+-------------------------------+\n|Fish           |3              |\n+-------------------------------+\n|Apples         |5              |\n+-------------------------------+\n|Pizza          |13             |\n+-------------------------------+\n");

    //     println!("1-------10--------20--------30--------40--------50--------60--------70--------80");
    //     println!("''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|");
    //     println!("{}", output);

    //     assert_eq!(expected, output);
    // }
}