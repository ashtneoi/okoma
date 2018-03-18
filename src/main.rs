extern crate rand;

fn main() {
    hello();
}


fn hello() {
    println!("Today I have {} legs", rand::random::<u8>());
}
