#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No day specified!");
    }
    let day = &args[1].parse::<u32>();

    if let Ok(day) = *day {
        let result: (String, String) = match day {
            1 => day01::solve(),
            2 => day02::solve(),
            3 => day03::solve(),
            4 => day04::solve(),
            5 => day05::solve(),
            6 => day06::solve(),
            7 => day07::solve(),
            8 => day08::solve(),
            _ => panic!("No day between 1 and 8 specified!"),
        };

        println!("{:?}", result);
    }
}
