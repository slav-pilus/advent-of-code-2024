mod day_2;
mod day_1;
mod util;

use std::env;
use crate::day_2::day_2;
use crate::day_1::day_1;

fn main() {
    println!("\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a day");
        return;
    }

    match args[1].as_str() {
        "1" => day_1(),
        "2" => day_2(),
        _ => println!("Invalid day")
    }
}
