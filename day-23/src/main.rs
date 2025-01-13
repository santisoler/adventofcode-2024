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

    #[test]
    fn test_part_two() {
        let fname = "data/test_input";
        let result = solve_part_two(fname);
        assert_eq!(result, "co,de,ka,ta");
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

    /// Get the largest subnet in the network
    pub fn get_largest_subnet(&self) -> Set {
        let mut largest_subnet = vec![];
        for computer in self.net.keys() {
            let subnets = match self.get_largest_subnets(computer) {
                None => continue,
                Some(s) => s,
            };
            let subnet = subnets[0].clone();
            if subnet.len() > largest_subnet.len() {
                largest_subnet = subnet;
            }
        }
        largest_subnet
    }

    /// Get the largest subnets that a computer belongs to
    fn get_largest_subnets(&self, computer: &str) -> Option<Vec<Set>> {
        let connections = match self.get(computer) {
            None => panic!(),
            Some(c) => c,
        };
        let subnets = match self.get_subnets_recursively(computer, &connections) {
            Some(s) => s,
            None => return None,
        };
        let max_length = subnets.iter().map(|s| s.len()).max().unwrap();
        let largest_subnets = subnets
            .into_iter()
            .filter(|s| s.len() == max_length)
            .collect();
        Some(largest_subnets)
    }

    /// Recursive function to get the subnets of a given computer.
    /// The targets argument is a Set of computers to which we are going to verify if the given
    /// computer is connected to.
    fn get_subnets_recursively(&self, computer: &str, targets: &Set) -> Option<Vec<Set>> {
        let connections = match self.get(computer) {
            Some(c) => c,
            None => {
                if targets.contains(&computer.to_string()) {
                    let set = vec![computer.to_string()];
                    return Some(vec![set]);
                };
                return None;
            }
        };
        // Keep only the connections that are also in targets
        let connections: Vec<String> = connections
            .into_iter()
            .filter(|c| targets.contains(&c.to_string()))
            .map(|c| c.to_string())
            .collect();
        if connections.is_empty() {
            return None;
        }

        let mut subnets = vec![];
        for connection in connections.iter() {
            let mut connections_subnets =
                match self.get_subnets_recursively(connection, &connections) {
                    Some(s) => s,
                    None => continue,
                };
            for s in connections_subnets.iter_mut() {
                s.push(computer.to_string());
            }
            subnets.extend(connections_subnets);
        }
        if subnets.is_empty() {
            return None;
        }
        Some(subnets)
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

fn solve_part_two(fname: &str) -> String {
    let network = parse_file(fname);
    let mut largest_subnet = network.get_largest_subnet();
    largest_subnet.sort();
    largest_subnet.join(",")
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());

    // let fname = "data/test_input";
    let start = Instant::now();
    let result = solve_part_two(fname);
    let end = Instant::now();
    println!("Solution to part two: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());
}
