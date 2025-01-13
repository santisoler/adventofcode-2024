use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;

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

pub struct Network {
    net: BTreeMap<String, Set>,
}

impl Network {
    pub fn new() -> Self {
        let net = BTreeMap::new();
        Self { net }
    }

    pub fn insert(&mut self, a: &str, b: &str) {
        // Sort the two computers alphabetically
        let mut pair = [a, b];
        pair.sort();
        let [a, b] = pair;
        self.net
            .entry(a.to_string())
            .and_modify(|set| set.push(b.to_string()))
            .or_insert(vec![b.to_string()]);
    }

    pub fn get(&self, a: &str) -> Option<&Set> {
        self.net.get(a)
    }

    fn are_connected(&self, a: &str, b: &str) -> bool {
        // Sort the two computers alphabetically
        let mut pair = [a, b];
        pair.sort();
        let [a, b] = pair;
        let connections = match self.get(a) {
            None => return false,
            Some(c) => c,
        };
        connections.contains(&b.to_string())
    }
}

/// Count the number of subnets of length 3 that includes a computer that starts with 't'.
fn count_subnets_len_3_with_t(network: &Network) -> u32 {
    let mut n_subnets = 0;
    for (computer, connections) in network.net.iter() {
        for (i, computer_a) in connections.iter().enumerate() {
            for computer_b in connections[i + 1..].iter() {
                let any_with_t = computer.starts_with('t')
                    || computer_a.starts_with('t')
                    || computer_b.starts_with('t');
                if any_with_t && network.are_connected(computer_a, computer_b) {
                    n_subnets += 1;
                }
            }
        }
    }
    n_subnets
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
    let network = parse_file(fname);
    count_subnets_len_3_with_t(&network)
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());
}
