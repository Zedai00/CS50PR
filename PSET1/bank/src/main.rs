use rust50::input;

fn main() {
    let greeting: String = input("Greeting: ");
    let greeting = greeting.to_lowercase();
    if greeting.starts_with("hello") {
        println!("$0");
    } else if greeting.starts_with("h") {
        println!("$20");
    } else {
        println!("$100");
    }
}
