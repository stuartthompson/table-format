mod data_item;
mod data_source;
mod vec_data_source;
mod table;

use table::{Table, TableColumn};
use data_item::DataItem;
use data_source::WSDataFrame;
use vec_data_source::VecDataSource;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_vector_table() {
        let mut data = VecDataSource::from(
            vec!("Fish", "3", "Apples", "5", "Pizza", "13"));

        let columns = vec!(
            TableColumn::fixed("Food".to_string(), 15),
            TableColumn::fixed("Count".to_string(), 15),
        );
        let table = Table::from(&mut data, columns);

        let output = table.format(80);

        let expected = String::from("+-------------------------------+\n|Food           |Count          |\n+-------------------------------+\n|Fish           |3              |\n+-------------------------------+\n|Apples         |5              |\n+-------------------------------+\n|Pizza          |13             |\n+-------------------------------+\n");

        println!("1-------10--------20--------30--------40--------50--------60--------70--------80");
        println!("''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|");
        println!("{}", output);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_multiline_data_source_table() {
        let mut data = WSDataFrame::from(String::from("asdfasds"));

        let columns = vec!(
            TableColumn::fixed("Byte 1".to_string(), 15),
            TableColumn::fixed("Byte 2".to_string(), 15),
            TableColumn::fixed("Byte 3".to_string(), 15),
        );
        let table = Table::from(&mut data, columns);

        let output = table.format(80);

        let expected = String::from("+-------------------------------+\n|Food           |Count          |\n+-------------------------------+\n|Fish           |3              |\n+-------------------------------+\n|Apples         |5              |\n+-------------------------------+\n|Pizza          |13             |\n+-------------------------------+\n");

        println!("1-------10--------20--------30--------40--------50--------60--------70--------80");
        println!("''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|''''5''''|");
        println!("{}", output);

        assert_eq!(expected, output);
    }
}