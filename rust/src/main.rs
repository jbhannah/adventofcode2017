use std::env;
use std::process;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No day specified!");
        process::exit(1);
    }
    let day = &args[1].parse::<u32>();

    if let Ok(day) = *day {
        match day {
            1 => day01::solve(),
            2 => day02::solve(),
            3 => day03::solve(),
            4 => day04::solve(),
            5 => day05::solve(),
            6 => day06::solve(),
            _ => println!("No day between 1-6 specified!")
        };
    }
}
