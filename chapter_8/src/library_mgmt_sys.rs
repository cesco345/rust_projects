use std::collections::{HashMap, HashSet};

// Define core structures
#[derive(Clone, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    copies: i32,
}

#[derive(Debug)]
struct Member {
    name: String,
    borrowed_books: HashSet<String>, // Set of ISBNs
}

struct Library {
    books: HashMap<String, Book>,        // ISBN -> Book
    members: HashMap<String, Member>,    // Member ID -> Member
    loans: HashMap<String, Vec<String>>, // ISBN -> Vec<Member ID>
}

impl Library {
    // Initialize a new library
    fn new() -> Library {
        Library {
            books: HashMap::new(),
            members: HashMap::new(),
            loans: HashMap::new(),
        }
    }

    // Add a new book to the library
    fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }

    // Register a new member
    fn add_member(&mut self, member_id: String, name: String) {
        self.members.insert(
            member_id,
            Member {
                name,
                borrowed_books: HashSet::new(),
            },
        );
    }

    // Attempt to borrow a book
    fn borrow_book(&mut self, member_id: &str, isbn: &str) -> Result<(), String> {
        // Check if book exists and has available copies
        let book = self.books.get_mut(isbn).ok_or("Book not found")?;

        if book.copies <= 0 {
            return Err("No copies available".to_string());
        }

        // Check if member exists
        let member = self.members.get_mut(member_id).ok_or("Member not found")?;

        // Check if member already has this book
        if member.borrowed_books.contains(isbn) {
            return Err("Member already has this book".to_string());
        }

        // Update book count
        book.copies -= 1;

        // Record the loan
        member.borrowed_books.insert(isbn.to_string());
        self.loans
            .entry(isbn.to_string())
            .or_insert_with(Vec::new)
            .push(member_id.to_string());

        Ok(())
    }

    // Return a book
    fn return_book(&mut self, member_id: &str, isbn: &str) -> Result<(), String> {
        // Check if book exists
        let book = self.books.get_mut(isbn).ok_or("Book not found")?;

        // Check if member exists and has borrowed the book
        let member = self.members.get_mut(member_id).ok_or("Member not found")?;

        if !member.borrowed_books.remove(isbn) {
            return Err("Member hasn't borrowed this book".to_string());
        }

        // Update book count
        book.copies += 1;

        // Remove loan record
        if let Some(loans) = self.loans.get_mut(isbn) {
            loans.retain(|id| id != member_id);
        }

        Ok(())
    }

    // Get all books borrowed by a member
    fn get_member_books(&self, member_id: &str) -> Vec<&Book> {
        if let Some(member) = self.members.get(member_id) {
            member
                .borrowed_books
                .iter()
                .filter_map(|isbn| self.books.get(isbn))
                .collect()
        } else {
            Vec::new()
        }
    }
}

fn main() {
    // Initialize library
    let mut library = Library::new();

    // Add some books
    let books = vec![
        Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            isbn: "001".to_string(),
            copies: 2,
        },
        Book {
            title: "Programming Rust".to_string(),
            author: "Jim Blandy".to_string(),
            isbn: "002".to_string(),
            copies: 1,
        },
    ];

    for book in books {
        library.add_book(book);
    }

    // Add members
    library.add_member("M001".to_string(), "Alice".to_string());
    library.add_member("M002".to_string(), "Bob".to_string());

    // Demonstrate book borrowing
    println!("Borrowing attempts:");
    match library.borrow_book("M001", "001") {
        Ok(()) => println!("Alice successfully borrowed book 001"),
        Err(e) => println!("Error: {}", e),
    }

    match library.borrow_book("M002", "001") {
        Ok(()) => println!("Bob successfully borrowed book 001"),
        Err(e) => println!("Error: {}", e),
    }

    // Try to borrow unavailable book
    match library.borrow_book("M002", "002") {
        Ok(()) => println!("Bob successfully borrowed book 002"),
        Err(e) => println!("Error: {}", e),
    }

    // Print borrowed books for Alice
    println!("\nAlice's borrowed books:");
    for book in library.get_member_books("M001") {
        println!("{} by {}", book.title, book.author);
    }

    // Return a book
    match library.return_book("M001", "001") {
        Ok(()) => println!("\nAlice successfully returned book 001"),
        Err(e) => println!("\nError returning book: {}", e),
    }
}
