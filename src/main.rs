/// This is a linked list implementation in Rust for learning. 
// Item is an item in our linked list implementation
struct Item<'a>{
    value: &'a str,
    left: Option<Box<Item<'a>>>,
    right: Option<Box<Item<'a>>>,
}

impl Item <'static>{
    fn insert(&mut self) -> Option<Box<Item>> {
        panic!("not impl")
    }
    fn search(&mut self) ->Option<Box<Item>> {
        panic!("not impl")
    }
} 

fn main() {
    println!("Hello, world!");
}
