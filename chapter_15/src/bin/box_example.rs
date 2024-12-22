// Example of recursive type with boxes
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Basic Box usage
    let b = Box::new(5);
    println!("Basic box value: {}", b);

    // Recursive type with boxes
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Recursive list: {:?}", list);
}
