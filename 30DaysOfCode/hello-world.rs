use std::io;

fn main() {
    println!("Hello, World.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);
}
