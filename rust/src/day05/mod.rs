use util;

pub fn solve() {
    let contents = util::read_input(5);
    let mut instrs: Vec<i32> = Vec::new();

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
