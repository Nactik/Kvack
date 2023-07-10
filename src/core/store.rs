mod store;

pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn insert(&mut self, key: &str, value: String) -> Result<String, String>;
    fn delete(&mut self, key: &str) -> Result<String, String>;
    fn update(&mut self, key: &str, value: String)) -> Result<String, String>;
}
