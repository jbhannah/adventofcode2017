use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("../input/day01.txt").expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Could not read file!");

    let data = contents.lines().rev().last().unwrap();
    let values: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    println!("{}", sum1(values.clone()));
    println!("{}", sum2(values.clone()));
}

fn sum1(values: Vec<u32>) -> u32 {
    let mut sum = 0;
    let mut curr = values.last().unwrap();

    for next in &values {
        if curr == next {
            sum += next;
        }

        curr = next;
    }

    return sum;
}

fn sum2(values: Vec<u32>) -> u32 {
    let mut sum = 0;
    let mut index = 0;
    let max = values.len() / 2;

    while index < max {
        let curr = values[index];
        let next = values[index + max];

        if curr == next {
            sum += curr * 2;
        }

        index += 1;
    }

    return sum;
}
