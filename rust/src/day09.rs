use util;

pub fn solve() -> (String, String) {
    let contents = util::read_input(9);
    let data = contents.lines().last().unwrap();

    let stream: Vec<char> = data.chars().collect();
    let (stream_cleaned, garbage_count) = clean_stream(&stream);
    let score = get_score(&stream_cleaned);

    return (score.to_string(), garbage_count.to_string());
}

fn clean_stream(stream: &Vec<char>) -> (Vec<char>, u32) {
    let mut stream_cleaned: Vec<char> = Vec::new();
    let mut garbage_count = 0;
    let mut garbage = false;
    let mut skip = false;

    for c in stream {
        if skip {
            skip = !skip;
            continue;
        }

        match *c {
            '!' => skip = !skip,
            '>' => garbage = false,
            _ => {
                if garbage {
                    garbage_count += 1;
                } else if *c == '<' {
                    garbage = true;
                } else {
                    stream_cleaned.push(*c);
                }
            }
        }
    }

    return (stream_cleaned.clone(), garbage_count);
}

fn get_score(stream: &Vec<char>) -> u32 {
    let mut score = 0;
    let mut curr_score = 0;

    for c in stream {
        match *c {
            '{' => curr_score += 1,
            '}' => {
                score += curr_score;
                curr_score -= 1;
            }
            _ => continue,
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09() {
        let (part1, part2) = solve();
        assert_eq!(14204.to_string(), part1);
        assert_eq!(6622.to_string(), part2);
    }
}
