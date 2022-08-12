use rust50::input;

fn main() {
    let dollars = dollars_to_float(input("How much was the meal?: "));
    let percent = percent_to_float(input("What percentage would you like to tip?: "));
    let tip = dollars * percent;
    println!("Leave: ${:.2}", tip);
}

fn dollars_to_float(d: String) -> f32 {
    return d.replace("$", "").parse::<f32>().expect("Invalid Number!");
}

fn percent_to_float(p: String) -> f32 {
    return p
        .replace("%", "")
        .parse::<f32>()
        .expect("Invalid Percentage!") / 100f32;
}
