use super::content::Content;

pub trait TableDataSource {
    fn get_data_iterator() -> dyn Iterator<Item = Vec<Content>>;
}