// Struct
#[derive(Debug)]
struct Node {
    next: Option<&mut Node>,
    value: String,
    key: i32,
}

#[derive(Debug)]
pub struct Btree {
    root: Option<Box<Node>>,
    size: usize,
    balance: i32,
}

// Methods

impl Node {
    fn new() -> Node {
        Node {
            key: 0,
            next: None,
            value: "".to_string(),
        }
    }

    fn is_leaf(&self) -> bool {
        return !self.value.is_empty();
    }
}

impl Btree {
    fn find(&self, key: &u32) -> Option<u32> {
        Some(0)
    }

    pub fn get(&self, key: &u32) -> Option<u32> {
        Some(0)
    }

    pub fn insert(&mut self, key: &u32, value: String) -> Result<String, String> {
        Ok("".to_string())
    }

    pub fn delete(&mut self, key: &u32) -> Result<u32, String> {
        Ok(0)
    }

    pub fn update(&mut self, key: &u32, value: String) -> Result<String, String> {
        Ok("".to_string())
    }

    pub fn new() -> Self {
        Btree {
            root: None,
            size: 0,
            balance: 4, // balance = 1 + (size of ram / size of key type )
        }
    }
}
