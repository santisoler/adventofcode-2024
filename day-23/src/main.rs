use std::collections::BTreeMap;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 7);
    }
}

type Set = Vec<String>;

struct Network {
    net: BTreeMap<String, Set>,
}

impl Network {
    fn new() -> Self {
        let net = BTreeMap::new();
        Self { net }
    }

    fn insert(&mut self, a: &str, b: &str) {
        // Sort the two computers alphabetically
        let mut pair = [a, b];
        pair.sort();
        let [a, b] = pair;
        self.net
            .entry(a.to_string())
            .and_modify(|set| set.push(b.to_string()))
            .or_insert(vec![b.to_string()]);
    }

    fn get(&self, a: &str) -> Option<&Set> {
        self.net.get(a)
    }
}

fn find_loops_recursive(
    network: &Network,
    computer: &str,
    targets: &Vec<String>,
    loop_len: usize,
) -> Option<Vec<Set>> {
    // use 1 to make the loop_len match the len of the loop
    if loop_len == 1 {
        if targets.contains(&computer.to_string()) {
            let set = vec![computer.to_string()];
            return Some(vec![set]);
        };
        return None;
    };
    let mut loops = vec![vec![]];
    let connections = match network.get(computer) {
        None => return None,
        Some(c) => c,
    };
    for connection in connections.iter() {
        let mut sets = match find_loops_recursive(network, connection, targets, loop_len - 1) {
            None => continue,
            Some(sets) => sets,
        };
        for set in sets.iter_mut() {
            set.push(computer.to_string())
        }
        loops.extend(sets);
    }
    loops = loops
        .into_iter()
        .filter(|set| set.len() == loop_len)
        .collect();
    Some(loops)
}

fn find_loops(network: &Network, loop_len: usize) -> Vec<Set> {
    let mut loops = vec![];
    for computer in network.net.keys() {
        let targets = match network.get(&computer) {
            Some(t) => t,
            None => continue,
        };
        let new_loops = match find_loops_recursive(&network, &computer, targets, loop_len) {
            None => continue,
            Some(l) => l,
        };
        loops.extend(new_loops);
    }
    loops
}

fn parse_file(fname: &str) -> Network {
    let content = fs::read_to_string(fname).unwrap();
    let mut network = Network::new();
    for line in content.lines() {
        let mut parts = line.split("-");
        let (a, b) = (parts.next().unwrap(), parts.next().unwrap());
        network.insert(a, b);
    }
    network
}

fn solve_part_one(fname: &str) -> u32 {
    let loop_len = 3;
    let network = parse_file(fname);
    let loops = find_loops(&network, loop_len);

    let mut n_loops_with_t = 0;
    for set in loops.iter() {
        for computer in set.iter() {
            let first_char = computer.chars().next().unwrap();
            if first_char == 't' {
                n_loops_with_t += 1;
                break;
            }
        }
    }
    n_loops_with_t
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
