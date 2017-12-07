use std::collections::HashMap;
use util;

enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

pub fn solve() {
    let contents = util::read_input(3);
    let data = contents.lines().rev().last().unwrap();
    let input: u32 = data.parse::<u32>().unwrap();

    println!("{}", steps_to_addr(input));
    println!("{}", next_largest(input));
}

fn next_largest(input: u32) -> u32 {
    let mut addr = 0;
    let mut coords = (0i32, 1i32);
    let mut direction = Direction::DOWN;
    let mut memory: HashMap<(i32, i32), u32> = HashMap::new();

    while addr < input {
        coords = move_forward(&coords, &direction);
        addr = sum_around(&coords, &memory);

        memory.insert(coords.clone(), addr.clone());

        if can_turn_left(&coords, &direction, &memory) {
            direction = turn_left(direction);
        }
    }

    return addr;
}

fn sum_around(coords: &(i32, i32), memory: &HashMap<(i32, i32), u32>) -> u32 {
    let mut sum = 0;
    let (x, y): (i32, i32) = *coords;
    let x_vals = [x - 1, x, x + 1];
    let y_vals = [y - 1, y, y + 1];

    for i in &x_vals {
        for j in &y_vals {
            let value = memory.get(&(*i, *j));
            if let Some(value) = value {
                sum += value;
            }
        }
    }

    if sum == 0 {
        sum = 1;
    }

    return sum;
}

fn steps_to_addr(input: u32) -> u32 {
    let mut addr = 0;
    let mut coords = (0i32, 1i32);
    let mut direction = Direction::DOWN;
    let mut memory: HashMap<(i32, i32), u32> = HashMap::new();

    while addr < input {
        addr += 1;
        coords = move_forward(&coords, &direction);

        memory.insert(coords.clone(), addr.clone());

        if can_turn_left(&coords, &direction, &memory) {
            direction = turn_left(direction);
        }
    }

    let (x, y) = coords;
    return (x.abs() + y.abs()) as u32;
}

fn can_turn_left(coords: &(i32, i32), direction: &Direction, memory: &HashMap<(i32, i32), u32>) -> bool {
    let (x, y): (i32, i32) = *coords;

    let left_coords: (i32, i32) = match *direction {
        Direction::UP => (x - 1, y),
        Direction::LEFT => (x, y - 1),
        Direction::DOWN => (x + 1, y),
        Direction::RIGHT => (x, y + 1)
    };

    return !memory.contains_key(&left_coords);
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::UP => Direction::LEFT,
        Direction::LEFT => Direction::DOWN,
        Direction::DOWN => Direction::RIGHT,
        Direction::RIGHT => Direction::UP
    }
}

fn move_forward(coords: &(i32, i32), direction: &Direction) -> (i32, i32) {
    let (x, y): (i32, i32) = *coords;

    match *direction {
        Direction::UP => (x, y + 1),
        Direction::LEFT => (x - 1, y),
        Direction::DOWN => (x, y - 1),
        Direction::RIGHT => (x + 1, y)
    }
}
