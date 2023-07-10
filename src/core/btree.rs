use crate::store::Store;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    next: Option<Box<Node>>,
    value: String,
    key: i32,
}

struct Btree {
    root: Node,
    size: usize,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            left: None,
            right: None,
            next: None,
            value,
        }
    }
}

impl Store for Btree {
    fn get(&self, key: &str) -> Option<String> {
        None
    }

    fn insert(&mut self, key: &str, value: String) -> Result<String, String> {
        Ok("".to_string())
    }

    fn delete(&mut self, key: &str) -> Result<String, String> {
        Ok("".to_string())
    }

    fn update(&mut self, key: &str, value: String) -> Result<String, String> {
        Ok("".to_string())
    }
}
