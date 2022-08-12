use std::process::exit;

use rust50::input;

fn main() {
    let inp: String = input("Expression: ");
    let inp: Vec<&str> = inp.split(" ").collect();
    let x = inp[0].parse::<f32>().unwrap();
    let z = inp[1];
    let y = inp[2].parse::<f32>().unwrap();
    let result = match calc(x, y, z) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
    println!("Result: {:.1}", result);
}

fn calc(x: f32, y: f32, z: &str) -> Result<f32, &str> {
    match z {
        "+" => Ok(x + y),
        "-" => Ok(x - y),
        "*" => Ok(x * y),
        "/" => Ok(x / y),
        _ => Err("Invalid Expression"),
    }
}
