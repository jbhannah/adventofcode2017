use util;

pub fn solve() {
    let contents = util::read_input(6);
    let data = contents.lines().rev().last().unwrap();

    let mut cycles = 0;
    let mut history: Vec<Vec<u32>> = Vec::new();
    let mut banks: Vec<u32> = data.split("\t").map(|n| n.parse::<u32>().unwrap()).collect();
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
            println!("{}", cycles);
            println!("{}", cycles - len);
            break;
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
