use util;

pub fn solve() -> (String, String) {
    let contents = util::read_input(1);

    let data = contents.lines().rev().last().unwrap();
    let values: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let s1 = sum1(values.clone());
    let s2 = sum2(values.clone());
    return (s1.to_string(), s2.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01() {
        let (part1, part2) = solve();
        assert_eq!(1228.to_string(), part1);
        assert_eq!(1238.to_string(), part2);
    }
}
