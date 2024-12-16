fn main() {
    let x = 5;
    let name = "Cesco";
    println!("Hello World!");
    println!("x is {}", x);
    println!("My name is {}", name);
    println!("Hello, {}!", name.to_uppercase());
    // this is not allowed, because name is immutable
    //name = "hello";
}
