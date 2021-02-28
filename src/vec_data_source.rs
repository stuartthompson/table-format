use super::data_item::DataItem;
use super::table::line::Line;

pub struct VecDataSource<T> {
    current_ix: usize,
    data: Vec<T>
}

impl<T> VecDataSource<T>
    where T: std::fmt::Display {
    pub fn from(data: Vec<T>) -> VecDataSource<T> {
        VecDataSource { current_ix: 0, data }
    }
}

impl<T> Iterator for VecDataSource<T>
    where T: std::fmt::Display {
    type Item = DataItem;
    fn next(&mut self) -> Option<DataItem> {
        if self.current_ix < self.data.len() {

            let item = DataItem {
                lines: vec!(Line::from(format!("{}", self.data[self.current_ix])))
            };

            self.current_ix += 1;

            Some(item)
        } else {
            None
        }
    }
}