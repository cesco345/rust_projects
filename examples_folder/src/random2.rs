extern crate rand;
use rand::RngCore;
fn main() {
    // get some random data:
    let mut data = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut data);
    println!("{:?}", data)
}
