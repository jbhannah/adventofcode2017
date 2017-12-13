use util;

pub fn solve() -> (String, String) {
    let contents = util::read_input(2);
    let mut lines: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let mut l: Vec<u32> = line.split("\t")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        l.sort();
        lines.push(l);
    }

    return (checksum(&lines).to_string(), divisum(&lines).to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02() {
        let (part1, part2) = solve();
        assert_eq!(36766.to_string(), part1);
        assert_eq!(261.to_string(), part2);
    }
}
