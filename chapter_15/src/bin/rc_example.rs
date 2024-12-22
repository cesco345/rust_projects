use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Reference count after creating b: {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
    }

    println!(
        "Reference count after c goes out of scope: {}",
        Rc::strong_count(&a)
    );
}
