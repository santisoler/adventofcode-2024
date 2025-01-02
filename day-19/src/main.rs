use std::cmp::min;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two() {
        let fname = "data/test_input";
        let result = solve_part_two(fname);
        assert_eq!(result, 16);
    }
}

#[derive(Clone, Debug)]
pub struct Patterns {
    patterns: HashSet<String>,
    max_stripes: usize,
}

/// HasSet with the available patterns.
impl Patterns {
    pub fn new() -> Self {
        Patterns {
            patterns: HashSet::new(),
            max_stripes: 0,
        }
    }

    pub fn new_from(patterns: Vec<String>) -> Self {
        let mut new = Self::new();
        for pattern in patterns {
            new.add(pattern);
        }
        new
    }

    pub fn add(&mut self, pattern: String) {
        let n_stripes = pattern.len();
        if self.patterns.insert(pattern) && self.max_stripes < n_stripes {
            self.max_stripes = n_stripes
        }
    }

    /// Check if a given design is possible to be created with the available patterns.
    pub fn is_possible(&self, design: &str) -> bool {
        if design.is_empty() {
            return true;
        }
        let max_stripes = min(self.max_stripes, design.len());
        for n_colors in 1..=max_stripes {
            // Check if there's a pattern we can use to start building the design. If so, run this
            // recursively, checking if the rest of the design is also possible.
            if self.patterns.contains(&design[0..n_colors]) {
                if self.is_possible(&design[n_colors..]) {
                    return true;
                }
            }
        }
        false
    }

    /// Count in how many possible ways a design can be built with the available patterns.
    pub fn count_possible_ways(&self, design: &str) -> u32 {
        if design.is_empty() {
            return 1;
        }
        let mut n_ways = 0;
        let max_stripes = min(self.max_stripes, design.len());
        for n_colors in 1..=max_stripes {
            if self.patterns.contains(&design[0..n_colors]) {
                n_ways += self.count_possible_ways(&design[n_colors..]);
            }
        }
        n_ways
    }
}

fn read_file(fname: &str) -> (Patterns, Vec<String>) {
    let content = fs::read_to_string(fname).unwrap();
    let mut lines = content.lines();
    let patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    let patterns = Patterns::new_from(patterns);
    let designs: Vec<String> = lines
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();
    (patterns, designs)
}

fn solve_part_one(fname: &str) -> u32 {
    let (patterns, designs) = read_file(fname);
    let n_possible_designs = designs
        .iter()
        .map(|d| patterns.is_possible(&d))
        .filter(|is_possible| *is_possible)
        .map(|b| b as u32)
        .sum();
    n_possible_designs
}

fn solve_part_two(fname: &str) -> u32 {
    let (patterns, designs) = read_file(fname);
    let n_possible_ways = designs
        .iter()
        .map(|d| patterns.count_possible_ways(&d))
        .sum();
    n_possible_ways
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {:.2e}s", (end - start).as_secs_f64());

    let fname = "data/test_input";
    let start = Instant::now();
    let result = solve_part_two(fname);
    let end = Instant::now();
    println!("Solution to part two: {result}");
    println!("Elapsed time: {:.2e}s", (end - start).as_secs_f64());
}
