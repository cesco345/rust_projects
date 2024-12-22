// Define a trait for items that can be borrowed from the library
pub trait LibraryItem {
    fn title(&self) -> &str;
    fn is_available(&self) -> bool;
    fn checkout(&mut self);
    fn return_item(&mut self);
}

// Book struct implementing LibraryItem
pub struct Book {
    title: String,
    author: String,
    available: bool,
}

impl Book {
    pub fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            available: true,
        }
    }
}

// Implement the LibraryItem trait for Book
impl LibraryItem for Book {
    fn title(&self) -> &str {
        &self.title
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn checkout(&mut self) {
        if self.available {
            self.available = false;
            println!("Book '{}' has been checked out.", self.title);
        } else {
            println!("Book '{}' is not available.", self.title);
        }
    }

    fn return_item(&mut self) {
        if !self.available {
            self.available = true;
            println!("Book '{}' has been returned.", self.title);
        }
    }
}

// DVD struct implementing LibraryItem
pub struct DVD {
    title: String,
    duration_mins: u32,
    available: bool,
}

impl DVD {
    pub fn new(title: String, duration_mins: u32) -> DVD {
        DVD {
            title,
            duration_mins,
            available: true,
        }
    }
}

// Implement LibraryItem for DVD
impl LibraryItem for DVD {
    fn title(&self) -> &str {
        &self.title
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn checkout(&mut self) {
        if self.available {
            self.available = false;
            println!("DVD '{}' has been checked out.", self.title);
        } else {
            println!("DVD '{}' is not available.", self.title);
        }
    }

    fn return_item(&mut self) {
        if !self.available {
            self.available = true;
            println!("DVD '{}' has been returned.", self.title);
        }
    }
}

// Library struct to manage collection of items
pub struct Library {
    items: Vec<Box<dyn LibraryItem>>,
}

impl Library {
    pub fn new() -> Library {
        Library { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Box<dyn LibraryItem>) {
        self.items.push(item);
    }

    pub fn checkout_item(&mut self, title: &str) {
        for item in &mut self.items {
            if item.title() == title && item.is_available() {
                item.checkout();
                return;
            }
        }
        println!("Item '{}' not found or not available.", title);
    }

    pub fn return_item(&mut self, title: &str) {
        for item in &mut self.items {
            if item.title() == title && !item.is_available() {
                item.return_item();
                return;
            }
        }
        println!("Item '{}' not found or already returned.", title);
    }
}

fn main() {
    // Create a new library
    let mut library = Library::new();

    // Add some items to the library
    library.add_item(Box::new(Book::new(
        "The Rust Book".to_string(),
        "Rust Team".to_string(),
    )));
    library.add_item(Box::new(DVD::new("Introduction to Rust".to_string(), 120)));

    // Demonstrate checkout and return
    library.checkout_item("The Rust Book");
    library.checkout_item("The Rust Book"); // Should fail
    library.return_item("The Rust Book");
    library.checkout_item("The Rust Book"); // Should work again
}
