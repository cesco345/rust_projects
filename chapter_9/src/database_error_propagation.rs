use std::error::Error;
use std::fmt;
use std::result::Result;

// Custom error types for database operations
#[derive(Debug)]
enum DbError {
    ConnectionFailed(String),
    QueryFailed(String),
    ValidationFailed(String),
    TransactionFailed(Box<dyn Error>),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            DbError::QueryFailed(msg) => write!(f, "Query execution failed: {}", msg),
            DbError::ValidationFailed(msg) => write!(f, "Data validation failed: {}", msg),
            DbError::TransactionFailed(e) => write!(f, "Transaction failed: {}", e),
        }
    }
}

impl Error for DbError {}

// Simulated database structure
struct Database {
    connected: bool,
}

// Database methods with error handling
impl Database {
    fn new() -> Self {
        Database { connected: false }
    }

    fn connect(&mut self) -> Result<(), DbError> {
        // Simulate connection logic
        if !self.connected {
            self.connected = true;
            Ok(())
        } else {
            Err(DbError::ConnectionFailed("Already connected".to_string()))
        }
    }

    fn execute_query(&self, query: &str) -> Result<(), DbError> {
        // Check connection status
        if !self.connected {
            return Err(DbError::ConnectionFailed("Not connected".to_string()));
        }

        // Simulate query validation and execution
        if query.is_empty() {
            return Err(DbError::ValidationFailed("Empty query".to_string()));
        }

        // Simulate query execution
        if query.contains("ERROR") {
            return Err(DbError::QueryFailed("Invalid SQL".to_string()));
        }

        Ok(())
    }

    fn transaction<F>(&self, operations: F) -> Result<(), DbError>
    where
        F: Fn() -> Result<(), DbError>,
    {
        // Start transaction
        println!("Beginning transaction...");

        // Execute operations
        match operations() {
            Ok(()) => {
                println!("Committing transaction...");
                Ok(())
            }
            Err(e) => {
                println!("Rolling back transaction...");
                Err(DbError::TransactionFailed(Box::new(e)))
            }
        }
    }
}

fn main() {
    let mut db = Database::new();

    // Attempt database operations with error handling
    match db.connect() {
        Ok(()) => println!("Connected to database"),
        Err(e) => println!("Error: {}", e),
    }

    // Test transaction with multiple operations
    let result = db.transaction(|| {
        // Execute multiple queries
        db.execute_query("SELECT * FROM users")?;
        db.execute_query("INSERT INTO logs ERROR")?;
        Ok(())
    });

    match result {
        Ok(()) => println!("Transaction completed successfully"),
        Err(e) => println!("Transaction failed: {}", e),
    }
}
