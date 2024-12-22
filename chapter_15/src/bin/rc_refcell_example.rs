use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("Initial list a: {:?}", a);
    println!("List b: {:?}", b);
    println!("List c: {:?}", c);

    // Modify the value through RefCell
    *value.borrow_mut() += 10;

    println!("\nAfter modification:");
    println!("Modified list a: {:?}", a);
    println!("Modified list b: {:?}", b);
    println!("Modified list c: {:?}", c);
}
