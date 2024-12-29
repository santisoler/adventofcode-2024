use std::fs;

pub struct Maze {
    pub map: Vec<Vec<char>>,
}

impl Maze {
    // Create new maze from input file
    pub fn new_from(fname: &str) -> Self {
        let content = fs::read_to_string(fname).unwrap();
        let mut map: Vec<Vec<char>> = vec![];
        for line in content.lines() {
            let row = line.chars().collect();
            map.push(row);
        }
        Self { map }
    }

    // Get start position
    pub fn get_start(&self) -> (usize, usize) {
        let j = self.map.len() - 2; // S is always in the previous to last row
        let i = self.map[j].iter().position(|c| *c == 'S').unwrap();
        (i, j)
    }

    // Get end position
    pub fn get_end(&self) -> (usize, usize) {
        let j = 1; // S is always in the second row
        let i = self.map[j].iter().position(|c| *c == 'E').unwrap();
        (i, j)
    }
}
