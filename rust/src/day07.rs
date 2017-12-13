use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use util;

struct Leaf {
    parent: RefCell<String>,
    weight: i32,
    children: Vec<String>,
}

impl Leaf {
    pub fn set_parent(&self, p: String) {
        *self.parent.borrow_mut() = p;
    }
}

pub fn solve() -> (String, String) {
    let contents = util::read_input(7);
    let mut tree: HashMap<String, Leaf> = HashMap::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<name>[a-z]+) \((?P<weight>\d+)\)(?: -> )?(?P<children>.+)?$").unwrap();
    }

    for line in contents.lines() {
        let caps = RE.captures(line).unwrap();

        let name: String = caps["name"].to_string();
        let weight: i32 = caps["weight"].parse::<i32>().unwrap();
        let mut children: Vec<String> = Vec::new();

        if let Some(c) = caps.name("children") {
            children = c.as_str().split(", ").map(|x| x.to_string()).collect();
        }

        tree.insert(
            name.clone(),
            Leaf {
                parent: RefCell::new(String::new()),
                weight: weight,
                children: children,
            },
        );
    }

    for (name, leaf) in tree.iter() {
        for c in &leaf.children {
            let child = tree.get(c).unwrap();
            child.set_parent(name.clone());
        }
    }

    let bottom = find_bottom(&tree);
    let (weight, _) = balance_weights(&tree, &bottom);
    return (bottom, weight.to_string());
}

fn find_bottom(tree: &HashMap<String, Leaf>) -> String {
    for (name, leaf) in tree {
        if leaf.parent.borrow().len() == 0 {
            return name.clone();
        }
    }

    return String::new();
}

fn balance_weights(tree: &HashMap<String, Leaf>, bottom: &String) -> (i32, i32) {
    let b = tree.get(bottom).unwrap();
    let len = b.children.len();

    if len == 0 {
        return (b.weight, 0);
    } else {
        let mut weights: Vec<i32> = Vec::new();
        let mut last_weight: i32 = 0;
        let mut curr_weight: i32 = 0;
        let mut curr_x: i32 = 0;

        for child in &b.children {
            let (x, y): (i32, i32) = balance_weights(tree, &child);

            if y == -1 {
                return (x, y);
            }

            let weight: i32 = x + y;

            if curr_weight != weight && last_weight != curr_weight {
                return (curr_x + (weight - curr_weight), -1);
            }

            last_weight = curr_weight;
            curr_weight = weight;
            curr_x = x;

            weights.push(weight);
        }

        let weight: i32 = weights.iter().sum();
        return (b.weight, weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07() {
        let (part1, part2) = solve();
        assert_eq!("fbgguv".to_string(), part1);
        assert_eq!(1864.to_string(), part2);
    }
}
