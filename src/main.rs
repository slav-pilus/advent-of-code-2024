mod days;
mod utils;

use std::env;
use days::day_1;
use days::day_2;
use days::day_3;
use days::day_4;

fn main() {
    println!("\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a day");
        return;
    }

    match args[1].as_str() {
        "1" => day_1::run(),
        "2" => day_2::run(),
        "3" => day_3::run(),
        "4" => day_4::run(),
        _ => println!("Invalid day")
    }
}
