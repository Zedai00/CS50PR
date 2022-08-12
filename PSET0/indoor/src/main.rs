use std::io::{self, Write};

fn main() {
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    io::stdin()        .read_line(&mut text)
        .expect("Failed To Read The Input.");

    println!("Output: {}", text.to_lowercase());
}
