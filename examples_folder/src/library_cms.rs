#[derive(Debug)] // Enable the Debug trait for pretty printing
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_checked_out: bool, // A flag to indicate if the book is checked out
}

impl Book {
    // Associated function to create a new book instance
    fn new(title: &str, author: &str, pages: u32) -> Self {
        Self {
            title: String::from(title),
            author: String::from(author),
            pages,
            is_checked_out: false, // By default, the book is not checked out
        }
    }

    // Method to check out the book
    fn check_out(&mut self) {
        if self.is_checked_out {
            println!("The book '{}' is already checked out!", self.title);
        } else {
            self.is_checked_out = true;
            println!("You have successfully checked out '{}'.", self.title);
        }
    }

    // Method to return the book
    fn return_book(&mut self) {
        if !self.is_checked_out {
            println!("The book '{}' is not checked out!", self.title);
        } else {
            self.is_checked_out = false;
            println!("You have successfully returned '{}'.", self.title);
        }
    }

    // Method to display book details
    fn display(&self) {
        println!(
            "Title: '{}', Author: '{}', Pages: {}, Checked Out: {}",
            self.title, self.author, self.pages, self.is_checked_out
        );
    }
}

fn main() {
    // Create two new books using the associated function `new`
    let mut book1 = Book::new("The Rust Programming Language", "Steve Klabnik", 550);
    let mut book2 = Book::new("Rust in Action", "Tim McNamara", 400);

    // Display details of both books
    book1.display();
    book2.display();

    // Check out the first book
    book1.check_out();

    // Attempt to check out the first book again
    book1.check_out();

    // Return the first book
    book1.return_book();

    // Display details again to confirm status changes
    book1.display();
    book2.display();
}
