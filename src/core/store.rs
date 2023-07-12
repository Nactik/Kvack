mod store;

pub trait Store {
    fn get(&self, key: &u32) -> Option<u32>;
    fn insert(&mut self, key: &u32, value: String) -> Result<String, String>;
    fn delete(&mut self, key: &u32) -> Result<u32, String>;
    fn update(&mut self, key: &u32, value: String)) -> Result<String, String>;
    fn new() -> Self;
}
