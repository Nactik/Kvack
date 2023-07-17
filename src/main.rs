mod datastore;

use datastore::btree::Btree;

fn main() {
    let mut btree = Btree::new();
    println!("Btree: {:?}", btree);
}
