use rust50::input;

fn main() {
    let inp: String =
        input("What is the Answer to the Great Question of Life, the Universe and Everything?: ");
    let inp = inp.to_lowercase();
    if inp == "42" || inp == "forty-two" || inp == "forty two" {
        println!("Yes");
    } else {
        println!("No");
    }
}
