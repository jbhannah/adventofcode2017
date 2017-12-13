use util;

pub fn solve() -> (String, String) {
    let contents = util::read_input(4);
    let mut lines: Vec<Vec<Vec<u8>>> = Vec::new();

    for line in contents.lines() {
        let l: Vec<Vec<u8>> = line.split(" ").map(|s| s.as_bytes().to_vec()).collect();
        lines.push(l);
    }

    let (uniq, anag) = count_valid(&lines);
    return (uniq.to_string(), anag.to_string());
}

fn count_valid(lines: &Vec<Vec<Vec<u8>>>) -> (u32, u32) {
    let mut uniq = 0;
    let mut anag = 0;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        if is_uniq(line) {
            uniq += 1;
        }

        if is_anag(line) {
            anag += 1;
        }
    }

    return (uniq, anag);
}

fn is_uniq(line: &Vec<Vec<u8>>) -> bool {
    let mut l = line.clone();

    l.sort();
    l.dedup();

    return l.len() == line.len();
}

fn is_anag(line: &Vec<Vec<u8>>) -> bool {
    let mut l: Vec<Vec<u8>> = Vec::new();
    let iter = line.into_iter();

    for word in iter {
        let mut bytes = word.clone();
        bytes.sort_unstable();
        l.push(bytes);
    }

    return is_uniq(&l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04() {
        let (part1, part2) = solve();
        assert_eq!(383.to_string(), part1);
        assert_eq!(265.to_string(), part2);
    }
}
