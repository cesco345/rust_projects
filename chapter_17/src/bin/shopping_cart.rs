#[derive(Debug)]
pub struct Product {
    name: String,
    price: f64,
}

impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product { name, price }
    }
}

// Changed the State trait to take references and return new state
trait State {
    fn add_item(&self, items: &mut Vec<Product>, product: Product) -> Box<dyn State>;
    fn remove_item(&self, items: &mut Vec<Product>, product_name: &str) -> Box<dyn State>;
    fn checkout(&self, items: &Vec<Product>) -> Box<dyn State>;
}

struct EmptyState;

impl State for EmptyState {
    fn add_item(&self, items: &mut Vec<Product>, product: Product) -> Box<dyn State> {
        items.push(product);
        println!("First item added to cart!");
        Box::new(ActiveState)
    }

    fn remove_item(&self, _items: &mut Vec<Product>, _product_name: &str) -> Box<dyn State> {
        println!("Cannot remove from empty cart!");
        Box::new(EmptyState)
    }

    fn checkout(&self, _items: &Vec<Product>) -> Box<dyn State> {
        println!("Cannot checkout empty cart!");
        Box::new(EmptyState)
    }
}

struct ActiveState;

impl State for ActiveState {
    fn add_item(&self, items: &mut Vec<Product>, product: Product) -> Box<dyn State> {
        items.push(product);
        println!("Item added to cart!");
        Box::new(ActiveState)
    }

    fn remove_item(&self, items: &mut Vec<Product>, product_name: &str) -> Box<dyn State> {
        if let Some(pos) = items
            .iter()
            .position(|product| product.name == product_name)
        {
            items.remove(pos);
            println!("Item removed from cart!");
            if items.is_empty() {
                Box::new(EmptyState)
            } else {
                Box::new(ActiveState)
            }
        } else {
            println!("Item not found!");
            Box::new(ActiveState)
        }
    }

    fn checkout(&self, items: &Vec<Product>) -> Box<dyn State> {
        let total: f64 = items.iter().map(|product| product.price).sum();
        println!("Checking out... Total: ${:.2}", total);
        Box::new(CheckedOutState)
    }
}

struct CheckedOutState;

impl State for CheckedOutState {
    fn add_item(&self, _items: &mut Vec<Product>, _product: Product) -> Box<dyn State> {
        println!("Cannot add items to checked out cart!");
        Box::new(CheckedOutState)
    }

    fn remove_item(&self, _items: &mut Vec<Product>, _product_name: &str) -> Box<dyn State> {
        println!("Cannot remove items from checked out cart!");
        Box::new(CheckedOutState)
    }

    fn checkout(&self, _items: &Vec<Product>) -> Box<dyn State> {
        println!("Cart is already checked out!");
        Box::new(CheckedOutState)
    }
}

pub struct ShoppingCart {
    items: Vec<Product>,
    state: Box<dyn State>,
}

impl ShoppingCart {
    pub fn new() -> ShoppingCart {
        ShoppingCart {
            items: Vec::new(),
            state: Box::new(EmptyState),
        }
    }

    pub fn add_item(&mut self, product: Product) {
        let new_state = self.state.add_item(&mut self.items, product);
        self.state = new_state;
    }

    pub fn remove_item(&mut self, product_name: &str) {
        let new_state = self.state.remove_item(&mut self.items, product_name);
        self.state = new_state;
    }

    pub fn checkout(&mut self) {
        let new_state = self.state.checkout(&self.items);
        self.state = new_state;
    }

    pub fn display(&self) {
        println!("\nCart contents:");
        for item in &self.items {
            println!("{}: ${:.2}", item.name, item.price);
        }
        let total: f64 = self.items.iter().map(|product| product.price).sum();
        println!("Total: ${:.2}\n", total);
    }
}

fn main() {
    let mut cart = ShoppingCart::new();

    // Test empty cart
    println!("Testing empty cart:");
    cart.remove_item("Nonexistent");
    cart.checkout();

    // Test active cart
    println!("\nTesting active cart:");
    cart.add_item(Product::new("Book".to_string(), 29.99));
    cart.display();

    cart.add_item(Product::new("Laptop".to_string(), 999.99));
    cart.display();

    cart.remove_item("Book");
    cart.display();

    // Test checkout
    println!("\nTesting checkout:");
    cart.checkout();
    cart.add_item(Product::new("Mouse".to_string(), 24.99)); // Should fail
}
