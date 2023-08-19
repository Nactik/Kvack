// Struct
#[derive(Debug)]
struct Record {
    key: u32,
    value: String,
    next: Option<Node>,
}

#[derive(Debug)]
struct Node {
    records: Vec<Record>,
    keys: Vec<u32>,
    branching: u32, // must be better to pass it as global const
    children: Vec<Node>,
}

#[derive(Debug)]
pub struct Btree {
    root: Option<Node>,
    size: u32,
    branching: u32,
}

impl Record {
    fn new(key: u32, value: String, next: Option<Node>) -> Record {
        return Record {
            key: key,
            value: value,
            next: next,
        };
    }
}

// Methods
impl Node {
    fn new() -> Node {
        return Node {
            records: Vec::new(),
            keys: Vec::new(),
            children: Vec::new(),
            branching: 4,
        };
    }

    fn is_leaf(&self) -> bool {
        return !self.records.len() != 0;
    }
}

impl Btree {
    fn find_node(key: u32, node: Node) -> Option<Node> {
        if node.is_leaf() {
            return Some(node);
        } else {
            return Some(node);
        }
    }

    fn find(key: u32, node: Node) -> Option<Node> {
        let node_to_insert = Btree::find_node(key, node);
        return None;
    }

    pub fn get(&self, _key: &u32) -> Option<u32> {
        Some(0)
    }

    pub fn insert(&mut self, key: &u32, value: String) -> Result<String, String> {
        match self.root {
            Some(ref node) => {
                let mut node_to_insert = Btree::find(*key, *node);
                match node_to_insert {
                    Some(ref mut node) => {
                        node.records.push(Record::new(*key, value, None));
                        return Ok("".to_string());
                    }
                    None => return Err("Not yet implemented".to_string()),
                };
            }
            None => {
                let mut node = Node::new();
                node.keys.push(*key);
                node.records.push(Record::new(*key, value, None));
                self.root = Some(node);
                self.size += 1;
                return Ok("".to_string()); // TODO: think of a useful value to return
            }
        }
    }

    // pub fn delete(&mut self, key: &u32) -> Result<u32, String> {
    //     Ok(0)
    // }
    //
    // pub fn update(&mut self, key: &u32, value: String) -> Result<String, String> {
    //     Ok("".to_string())
    // }

    pub fn new() -> Self {
        return Btree {
            root: None,
            size: 0,
            branching: 4,
        };
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_insert_first_element() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!

        let mut tree = Btree::new();

        let result = tree.insert(&1, "test".to_string());

        let test = dbg!(&tree);
    }
}
