use super::content::Content;

pub struct DataItem {
    pub lines: Vec<Content>
}

impl std::str::FromStr for DataItem {
    type Err=std::fmt::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataItem { lines: vec!(Content::from_str(s).unwrap()) })
    }
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
