use std::collections::HashSet;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 6);
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
        for n_colors in 1..=self.max_stripes {
            // Don't check pattern if the design has less colors that current value of n_colors
            if design.len() < n_colors {
                continue;
            }
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
    // println!("{:?}", patterns);
    // for design in designs.iter() {
    //     println!("{}, {}", design, patterns.is_possible(&design));
    // }
    let n_possible_designs = designs
        .iter()
        .map(|d| patterns.is_possible(&d))
        .filter(|is_possible| *is_possible)
        .map(|b| b as u32)
        .sum();
    n_possible_designs
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
