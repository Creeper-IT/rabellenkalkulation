use std::fs::read;

fn main() {
    let test = 54;
    let test2:str;
    println!("Hello, world! {0} ", test);

    test2 = read!("{}\n");
    test = test2.parse::<i32>().unwrap();

    if text == 54 {
        println!("Die Zahl ist die 54");
    } else {
        println!("Die Zahl ist nicht die 54");
    }
}
