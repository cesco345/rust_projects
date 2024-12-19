use std::collections::HashMap;
use std::io::{self, Write};

// Main structure to hold company departments and employees
struct Company {
    // HashMap: Department name -> Vector of employee names
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    // Initialize a new company
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    // Add an employee to a department
    fn add_employee(&mut self, name: String, department: String) {
        // Get department or create new if doesn't exist
        let dept = self
            .departments
            .entry(department.to_lowercase())
            .or_insert_with(Vec::new);

        // Add employee and sort list
        dept.push(name);
        dept.sort();
    }

    // List all employees in a specific department
    fn list_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(&department.to_lowercase())
    }

    // List all departments and their employees
    fn list_all(&self) {
        // Get all department names and sort them
        let mut departments: Vec<&String> = self.departments.keys().collect();
        departments.sort();

        // Print each department and its employees
        for dept in departments {
            println!("\nDepartment: {}", dept);
            if let Some(employees) = self.departments.get(dept) {
                for employee in employees {
                    println!("  - {}", employee);
                }
            }
        }
    }
}

fn main() {
    let mut company = Company::new();

    loop {
        // Print menu
        println!("\nOptions:");
        println!("1. Add employee");
        println!("2. List department");
        println!("3. List all");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        // Read user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_employee_prompt(&mut company),
            "2" => list_department_prompt(&company),
            "3" => company.list_all(),
            "4" => break,
            _ => println!("Invalid option"),
        }
    }
}

// Handle employee addition user interaction
fn add_employee_prompt(company: &mut Company) {
    // Get employee name
    print!("Enter employee name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    // Get department name
    print!("Enter department: ");
    io::stdout().flush().unwrap();
    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();

    // Add employee to company
    company.add_employee(name.trim().to_string(), department.trim().to_string());
    println!("Employee added successfully!");
}

// Handle department listing user interaction
fn list_department_prompt(company: &Company) {
    // Get department name
    print!("Enter department name: ");
    io::stdout().flush().unwrap();
    let mut department = String::new();
    io::stdin().read_line(&mut department).unwrap();
    let department = department.trim().to_lowercase();

    // Display department employees
    match company.list_department(&department) {
        Some(employees) => {
            println!("\nEmployees in {}:", department);
            for employee in employees {
                println!("- {}", employee);
            }
        }
        None => println!("Department not found!"),
    }
}
