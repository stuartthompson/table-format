use super::content::Content;

pub struct DataItem {
    pub lines: Vec<Content>
}

impl DataItem {
    pub fn from(
        lines: Vec<Content>
    ) -> DataItem {
        let mut data_item = DataItem {
            lines: Vec::new()
        };

        for line in lines {
            data_item.lines.push(line);
        }

        data_item
    }
}
