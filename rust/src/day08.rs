use regex::Regex;
use std::collections::HashMap;
use util;

pub fn solve() {
    let contents = util::read_input(8);
    let mut regs: HashMap<String, i32> = HashMap::new();
    let mut max: i32 = i32::min_value();

    for line in contents.lines() {
        if let Some(val) = do_instr(line.to_string(), &mut regs) {
            if val > max {
                max = val;
            }
        }
    }

    println!("{}", get_max(&regs));
    println!("{}", max);
}

fn do_instr(instr: String, regs: &mut HashMap<String, i32>) -> Option<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<reg>[a-z]+) (?P<oper>inc|dec) (?P<amt>-?\d+) if (?P<regc>[a-z]+) (?P<cmp>\S+) (?P<expect>-?\d+)").unwrap();
    }

    let caps = RE.captures(instr.as_ref()).unwrap();

    let regc = caps["regc"].to_string();
    let regc_expect = caps["expect"].parse::<i32>().unwrap();
    let regc_val: i32;

    if let Some(v) = regs.get(&regc) {
        regc_val = *v;
    } else {
        regc_val = 0;
    }

    let cmp = match &caps["cmp"] {
        "==" => regc_val == regc_expect,
        "!=" => regc_val != regc_expect,
        ">" => regc_val > regc_expect,
        ">=" => regc_val >= regc_expect,
        "<" => regc_val < regc_expect,
        "<=" => regc_val <= regc_expect,
        _ => false,
    };

    if !cmp {
        return None;
    }

    let oper = caps["oper"].to_string();
    let amt = caps["amt"].parse::<i32>().unwrap();
    let delta: i32;

    if oper == "dec" {
        delta = amt * -1;
    } else {
        delta = amt;
    }

    let reg = caps["reg"].to_string();
    let reg_val = regs.entry(reg).or_insert(0);
    *reg_val += delta;

    return Some(*reg_val);
}

fn get_max(regs: &HashMap<String, i32>) -> i32 {
    let mut max: i32 = i32::min_value();

    for val in regs.values() {
        if *val > max {
            max = *val;
        }
    }

    return max;
}
