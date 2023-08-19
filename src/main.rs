mod datastore;

use datastore::btree::Btree;

fn main() {
    let mut btree = Btree::new();
    btree.insert(&1, "one".to_string());
    println!("Btree: {:?}", btree);
    let result: Result<String, String> = btree.insert(&2, "two".to_string());

    match result {
        Ok(_value) => println!("Ok"),
        Err(_error) => println!("{:?}", _error),
    }
    println!("Btree: {:?}", btree);
}
