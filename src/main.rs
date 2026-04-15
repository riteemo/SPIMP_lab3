use std::io;

fn main() {
    println!("Input:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("Output: {}", input.trim());
}