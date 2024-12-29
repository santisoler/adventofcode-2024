use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::directions::{Direction, Orientation};
use crate::maze::Maze;
use crate::tile::Tile;

mod directions;
mod maze;
mod tile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 7036);
    }
    #[test]
    fn test_part_one_example_two() {
        let fname = "data/test_input_2";
        let result = solve_part_one(fname);
        assert_eq!(result, 11048);
    }
}

const DIRECTIONS: [Direction; 3] = [Direction::Forward, Direction::Right, Direction::Left];

pub struct Visited {
    visited: [Vec<Vec<bool>>; 4],
}

impl Visited {
    pub fn new_from(maze: &Maze) -> Self {
        let nrows = maze.map.len();
        let ncols = maze.map[0].len();
        let visited_n = vec![vec![false; ncols]; nrows];
        let visited_s = vec![vec![false; ncols]; nrows];
        let visited_e = vec![vec![false; ncols]; nrows];
        let visited_w = vec![vec![false; ncols]; nrows];
        Self {
            visited: [visited_n, visited_e, visited_s, visited_w],
        }
    }

    pub fn was_visited(&self, x: usize, y: usize, orientation: Orientation) -> bool {
        self.visited[orientation.as_int() as usize][y][x]
    }

    pub fn visit(&mut self, x: usize, y: usize, orientation: Orientation) {
        self.visited[orientation.as_int() as usize][y][x] = true;
    }
}

pub struct Scores {
    scores: [Vec<Vec<u32>>; 4],
}

impl Scores {
    pub fn new_from(maze: &Maze) -> Self {
        let nrows = maze.map.len();
        let ncols = maze.map[0].len();
        let scores_n = vec![vec![u32::MAX; ncols]; nrows];
        let scores_s = vec![vec![u32::MAX; ncols]; nrows];
        let scores_e = vec![vec![u32::MAX; ncols]; nrows];
        let scores_w = vec![vec![u32::MAX; ncols]; nrows];
        Self {
            scores: [scores_n, scores_e, scores_s, scores_w],
        }
    }

    pub fn get(&self, x: usize, y: usize, orientation: Orientation) -> u32 {
        self.scores[orientation.as_int() as usize][y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, orientation: Orientation, score: u32) {
        self.scores[orientation.as_int() as usize][y][x] = score;
    }
}

fn get_lowest_score(fname: &str) -> Result<u32, &str> {
    // Read maze and get start and end positions
    let maze = Maze::new_from(fname);
    let start = maze.get_start();
    let end = maze.get_end();

    // Define structs to store scores of each tile and to mark if they were visited or not.
    // In these structs, each tile is defined by their position and their orientation.
    // Two tiles in the same location but different position should be treated as different.
    // Failing to do so would not result in the lowest score.
    let mut scores = Scores::new_from(&maze);
    let mut visited = Visited::new_from(&maze);

    // Initialize heap with the start tile.
    // We need to define the heap with elements of Reverse<Tile> so the heap is a min-heap, and not
    // a max-heap (as it is by default).
    let mut heap = BinaryHeap::<Reverse<Tile>>::new();
    let start_tile = Tile {
        x: start.0,
        y: start.1,
        score: 0,
        orientation: Orientation::East,
    };
    scores.write(
        start_tile.x,
        start_tile.y,
        start_tile.orientation,
        start_tile.score,
    );
    heap.push(Reverse(start_tile));

    while !heap.is_empty() {
        // Pop from heap (the tile with lowest score)
        let tile = heap.pop().unwrap().0;

        // Mark as visited
        visited.visit(tile.x, tile.y, tile.orientation);

        // If target tile, return score
        if (tile.x, tile.y) == end {
            return Ok(tile.score);
        }

        for direction in DIRECTIONS {
            // Get the neighboring tile
            let (x, y, orientation) = tile.get_neighbor(&direction);
            // Skip if neighbor is a wall or if it was already visited
            if maze.map[y][x] == '#' || visited.was_visited(x, y, orientation) {
                continue;
            };
            // Compute the score of the neighbor tile
            let score = match direction {
                Direction::Forward => tile.score + 1,
                Direction::Left | Direction::Right => tile.score + 1 + 1000,
            };
            // Override score if we found a smaller one.
            if score < scores.get(x, y, orientation) {
                let neighbor = Tile {
                    x,
                    y,
                    score,
                    orientation,
                };
                heap.push(Reverse(neighbor));
                scores.write(x, y, orientation, score)
            }
        }
    }
    Err("Couldn't find path")
}

fn solve_part_one(fname: &str) -> u32 {
    match get_lowest_score(fname) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
