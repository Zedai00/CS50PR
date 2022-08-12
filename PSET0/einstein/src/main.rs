use std::io::{self, Write};

fn main() {
    print!("Mass: ");
    io::stdout().flush().unwrap();
    let mut mass = String::new();
    io::stdin().read_line(&mut mass).unwrap();
    let mass: i64 = mass.trim().parse().expect("Invalid Input");
    let c: i64 = 90000000000000000;
    println!("Energy: {}", mass * c);
}
