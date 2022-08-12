use rust50::input;

fn main() {
    let time: String = input("What time is it?: ");
    let mut time = time.split(":");
    let h = time.next().unwrap().parse::<f32>().unwrap();
    let s = time.next().unwrap().parse::<f32>().unwrap();
    let time: f32 = h + s / 60.0;
    if time >= 7.0 && time <= 8.0 {
        println!("Breakfast Time")
    } else if time >= 12.0 && time <= 13.00 {
        println!("Lunch Time")
    } else if time >= 18.0 && time <= 19.0 {
        println!("Dinner Time")
    } else {
        println!()
    }
}
