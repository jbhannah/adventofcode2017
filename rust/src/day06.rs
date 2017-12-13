use util;

pub fn solve() -> (String, String) {
    let contents = util::read_input(6);
    let data = contents.lines().rev().last().unwrap();

    let mut cycles = 0;
    let mut history: Vec<Vec<u32>> = Vec::new();
    let mut banks: Vec<u32> = data.split("\t")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    history.push(banks.clone());

    loop {
        cycles += 1;
        let mut i = max_index(&banks);
        let max = banks[i];
        banks[i] = 0;
        let mut rem = max;

        while rem > 0 {
            i += 1;

            if i >= banks.len() {
                i = 0;
            }

            banks[i] += 1;
            rem -= 1;
        }

        if history.contains(&banks) {
            let len = history.iter().position(|&ref x| x == &banks).unwrap();
            return (cycles.to_string(), (cycles - len as u32).to_string());
        }

        history.push(banks.clone());
    }
}

fn max_index(banks: &Vec<u32>) -> usize {
    let mut i = 0;

    for (j, &value) in banks.iter().enumerate() {
        if value > banks[i] {
            i = j;
        }
    }

    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06() {
        let (part1, part2) = solve();
        assert_eq!(12841.to_string(), part1);
        assert_eq!(8038.to_string(), part2);
    }
}
