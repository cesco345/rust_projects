use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[2];
    let file_path = &args[3];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
