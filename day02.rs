use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("day02.txt").expect("File not found!");
    let mut contents = String::new();
    let mut lines: Vec<Vec<u32>> = Vec::new();

    f.read_to_string(&mut contents).expect("Could not read file!");

    for line in contents.lines() {
        let mut l: Vec<u32> = line.split("\t").map(|n| n.parse::<u32>().unwrap()).collect();
        l.sort();
        lines.push(l);
    }

    println!("{}", checksum(&lines));
    println!("{}", divisum(&lines));
}

fn checksum(lines: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for line in lines {
        sum += linesum(line);
    }

    return sum;
}

fn linesum(line: &Vec<u32>) -> u32 {
    return line.last().unwrap() - line.first().unwrap();
}

fn divisum(lines: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for line in lines {
        sum += linediv(line);
    }

    return sum;
}

fn linediv(line: &Vec<u32>) -> u32 {
    for i in line {
        for j in line {
            if j != i && j % i == 0 {
                return j / i;
            }
        }
    }

    return 0;
}
