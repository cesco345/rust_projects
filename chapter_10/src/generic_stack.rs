// A basic generic stack implementation demonstrating simple generic types
use std::fmt::Debug;

// Define a generic stack structure that can hold any type T
struct Stack<T> {
    items: Vec<T>,
}

// Implementation block for Stack with generic type T
impl<T> Stack<T> {
    // Create a new empty stack
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    // Push an item onto the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Pop an item from the stack, returning an Option
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek at the top item without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Check if stack is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Additional implementation for types that can be printed
impl<T: Debug> Stack<T> {
    // Print all items in the stack
    fn print_stack(&self) {
        println!("Stack contents: {:?}", self.items);
    }
}

fn main() {
    println!("Testing Integer Stack:");
    println!("---------------------");

    // Create a stack of integers
    let mut int_stack: Stack<i32> = Stack::new();

    // Test is_empty
    println!("Is stack empty? {}", int_stack.is_empty());

    // Push some numbers
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    // Print current stack
    println!("After pushing 1, 2, 3:");
    int_stack.print_stack();

    // Test peek
    if let Some(top) = int_stack.peek() {
        println!("Top element (peek): {}", top);
    }

    // Test pop
    if let Some(popped) = int_stack.pop() {
        println!("Popped element: {}", popped);
    }

    // Print stack after pop
    println!("After popping:");
    int_stack.print_stack();

    println!("\nTesting String Stack:");
    println!("--------------------");

    // Create a stack of strings
    let mut string_stack: Stack<String> = Stack::new();

    // Push some strings
    string_stack.push(String::from("Hello"));
    string_stack.push(String::from("World"));

    // Print initial state
    println!("Initial stack:");
    string_stack.print_stack();

    // Test peek on string stack
    if let Some(top) = string_stack.peek() {
        println!("Top element (peek): {}", top);
    }

    // Pop and print
    if let Some(popped) = string_stack.pop() {
        println!("Popped element: {}", popped);
    }

    // Final state
    println!("Final stack:");
    string_stack.print_stack();

    println!("Is stack empty? {}", string_stack.is_empty());
}
