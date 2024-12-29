use std::collections::HashMap;
use std::fs;

use crate::tile::{Orientation, Tile};
use crate::{SCORE_PER_STEP, SCORE_PER_TURN};

pub struct Maze {
    maze: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    scores: Vec<Vec<u32>>,
    pub predecessors: HashMap<(usize, usize), (usize, usize)>,
    ncols: usize,
    nrows: usize,
}

impl Maze {
    // Create new maze from input file
    pub fn new_from(fname: &str) -> Self {
        let content = fs::read_to_string(fname).unwrap();
        let mut maze: Vec<Vec<char>> = vec![];
        for line in content.lines() {
            let row = line.chars().collect();
            maze.push(row);
        }
        let nrows = maze.len();
        let ncols = maze[0].len();
        let visited = vec![vec![false; ncols]; nrows];
        let scores = vec![vec![u32::MAX; ncols]; nrows];
        let predecessors = HashMap::new();
        Self {
            maze,
            visited,
            scores,
            predecessors,
            ncols,
            nrows,
        }
    }

    // Get start and end positions in the maze
    pub fn get_start_and_end(&self) -> ((usize, usize), (usize, usize)) {
        let (mut start_x, mut start_y) = (usize::MAX, usize::MAX);
        let (mut end_x, mut end_y) = (usize::MAX, usize::MAX);
        for (j, row) in self.maze.iter().enumerate() {
            for (i, element) in row.iter().enumerate() {
                if *element == 'S' {
                    start_x = i;
                    start_y = j;
                } else if *element == 'E' {
                    end_x = i;
                    end_y = j;
                };
            }
        }
        if (start_x == usize::MAX) | (end_x == usize::MAX) {
            panic!("Couldn't find start and end tiles")
        }
        ((start_x, start_y), (end_x, end_y))
    }

    // Return the character in a given tile
    pub fn get_char(&self, x: usize, y: usize) -> char {
        self.maze[y][x]
    }

    // Return the score of a given tile
    pub fn get_score(&self, x: usize, y: usize) -> u32 {
        self.scores[y][x]
    }

    // Write score
    pub fn write_score(&mut self, x: usize, y: usize, value: u32) {
        self.scores[y][x] = value;
    }

    // Check if tile was visited
    pub fn was_visited(&self, x: usize, y: usize) -> bool {
        self.visited[y][x]
    }

    // Mark tile as visited
    pub fn visit(&mut self, x: usize, y: usize) {
        self.visited[y][x] = true;
    }

    pub fn get_unvisited_neighbors(&self, tile: &Tile) -> Vec<Tile> {
        let mut neighbors: Vec<Tile> = vec![];
        let directions = [
            Orientation::North,
            Orientation::South,
            Orientation::East,
            Orientation::West,
        ];
        for direction in directions {
            let (delta_x, delta_y) = direction.get_displacement();
            if ((tile.x == 0) && (delta_x < 0)) | ((tile.x == self.ncols - 1) && (delta_x > 0)) {
                continue;
            };
            if ((tile.y == 0) && (delta_y < 0)) | ((tile.y == self.nrows - 1) && (delta_y > 0)) {
                continue;
            };
            let x = (tile.x as i32 + delta_x) as usize;
            let y = (tile.y as i32 + delta_y) as usize;
            if self.was_visited(x, y) | (self.get_char(x, y) == '#') {
                continue;
            };
            let n_turns = tile.orientation.count_turns(&direction);
            let score = tile.score + SCORE_PER_STEP + SCORE_PER_TURN * n_turns;
            neighbors.push(Tile {
                x,
                y,
                score,
                orientation: direction,
            })
        }
        neighbors
    }

    pub fn print(&self, tile: Option<&Tile>) {
        for (j, row) in self.maze.iter().enumerate() {
            for (i, char) in row.iter().enumerate() {
                if let Some(t) = tile {
                    if (t.x == i) && (t.y == j) {
                        print!("@");
                        continue;
                    }
                }
                if self.was_visited(i, j) {
                    print!("x");
                } else {
                    print!("{}", char);
                };
            }
            print!("\n");
        }
    }

    pub fn print_path(&self) {
        let mut map = self.maze.clone();
        let (start, end) = self.get_start_and_end();
        let mut tile = end;
        while tile != start {
            map[tile.1][tile.0] = '@';
            tile = *self.predecessors.get(&tile).unwrap();
        }

        for row in map.iter() {
            for char in row.iter() {
                print!("{}", char);
            }
            print!("\n");
        }
    }
}
