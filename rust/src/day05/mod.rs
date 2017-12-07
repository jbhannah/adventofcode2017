use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("../input/day05.txt").expect("File not found!");
    let mut contents = String::new();
    let mut instrs: Vec<i32> = Vec::new();

    f.read_to_string(&mut contents).expect("Could not read file!");

    for line in contents.lines() {
        let i = line.parse::<i32>();
        if let Ok(i) = i {
            instrs.push(i);
        }
    }

    let inc_after_access = |i: i32| i + 1;
    let inc_or_dec_after_access = |i: i32| {
        if i < 3 {
            return i + 1;
        } else {
            return i - 1;
        }
    };

    println!("{}", steps_to_escape(instrs.clone(), inc_after_access));
    println!("{}", steps_to_escape(instrs.clone(), inc_or_dec_after_access));
}

fn steps_to_escape<F>(mut instrs: Vec<i32>, oper: F) -> u32
where F: Fn(i32) -> i32
{
    let mut steps = 0;
    let mut instr: i32 = 0;

    loop {
        if instr >= instrs.len() as i32 {
            break;
        }

        steps += 1;
        let curr_instr = instr as usize;
        let delta = instrs[curr_instr];
        instr += delta;

        instrs[curr_instr] = oper(delta);
    }

    return steps;
}
