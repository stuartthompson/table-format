use super::table::line::Line;

pub struct DataItem {
    pub lines: Vec<Line>
}

impl DataItem {
    pub fn from(
        lines: Vec<&str>
    ) -> DataItem {
        let mut data_item = DataItem {
            lines: Vec::new()
        };

        for line in lines {
            data_item.lines.push(Line::from(line.to_string()));
        }

        data_item
    }
}
