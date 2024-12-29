mod maze;
mod tile;
use std::io;

use crate::maze::Maze;
use crate::tile::{Orientation, Tile};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

const SCORE_PER_STEP: u32 = 1;
const SCORE_PER_TURN: u32 = 1000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_turns() {
        let orientation = Orientation::North;
        assert_eq!(orientation.count_turns(&Orientation::North), 0);
        assert_eq!(orientation.count_turns(&Orientation::South), 2);
        assert_eq!(orientation.count_turns(&Orientation::East), 1);
        assert_eq!(orientation.count_turns(&Orientation::West), 1);
        let orientation = Orientation::East;
        assert_eq!(orientation.count_turns(&Orientation::North), 1);
        assert_eq!(orientation.count_turns(&Orientation::South), 1);
        assert_eq!(orientation.count_turns(&Orientation::East), 0);
        assert_eq!(orientation.count_turns(&Orientation::West), 2);
        let orientation = Orientation::South;
        assert_eq!(orientation.count_turns(&Orientation::North), 2);
        assert_eq!(orientation.count_turns(&Orientation::South), 0);
        assert_eq!(orientation.count_turns(&Orientation::East), 1);
        assert_eq!(orientation.count_turns(&Orientation::West), 1);
        let orientation = Orientation::West;
        assert_eq!(orientation.count_turns(&Orientation::North), 1);
        assert_eq!(orientation.count_turns(&Orientation::South), 1);
        assert_eq!(orientation.count_turns(&Orientation::East), 2);
        assert_eq!(orientation.count_turns(&Orientation::West), 0);
    }

    #[test]
    fn test_part_one_example_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 7036)
    }

    #[test]
    fn test_part_one_example_two() {
        let fname = "data/test_input_2";
        let result = solve_part_one(fname);
        assert_eq!(result, 11048)
    }

    #[test]
    fn test_part_one_example_three() {
        let fname = "data/test_input_3";
        let result = solve_part_one(fname);
        assert_eq!(result, 4008)
    }

    #[test]
    fn test_part_one_example_four() {
        let fname = "data/test_input_4";
        let result = solve_part_one(fname);
        assert_eq!(result, 11 + 5 * 1000)
    }
}

// Compute minimum possible score through Dijkstra's algorithm
fn find_minimum_score(fname: &str) -> Result<u32, &str> {
    // Read file and build maze
    let mut maze = Maze::new_from(fname);

    // Get start and end tiles
    let (start, end) = maze.get_start_and_end();

    // Define heap.
    // Use reverse to create a min-heap instead of a max-heap
    // (default behaviour of the BinaryHeap).
    let mut heap: BinaryHeap<Reverse<Tile>> = BinaryHeap::new();

    // Push the start tile into the heap (and update score in the maze)
    let start = Tile {
        x: start.0,
        y: start.1,
        score: 0,
        orientation: Orientation::East,
    };
    maze.write_score(start.x, start.y, start.score);
    heap.push(Reverse(start)); // need to push a Reverse<Tile>

    while !heap.is_empty() {
        // Pop tile with smallest score.
        // Use the .0 to extract the Tile from the Reverse.
        let tile = heap.pop().unwrap().0;

        // If we found the end tile, return its score
        if (tile.x == end.0) && (tile.y == end.1) {
            maze.print_path();
            return Ok(tile.score);
        }

        // Mark tile as visited
        maze.visit(tile.x, tile.y);

        // maze.print(Some(&tile));
        // let mut answer = String::new();
        // io::stdin().read_line(&mut answer).unwrap();

        // Check unvisited neighbors
        for neighbor in maze.get_unvisited_neighbors(&tile) {
            // If we found a smaller score for a neighbor, add it to the heap and update its score
            // in the maze.
            if neighbor.score < maze.get_score(neighbor.x, neighbor.y) {
                maze.write_score(neighbor.x, neighbor.y, neighbor.score);
                maze.predecessors
                    .entry((neighbor.x, neighbor.y))
                    .and_modify(|e| *e = (tile.x, tile.y))
                    .or_insert((tile.x, tile.y));
                heap.push(Reverse(neighbor));
            }
        }
    }
    Err("Couldn't find path that leads to the end tile.")
}

fn solve_part_one(fname: &str) -> u32 {
    match find_minimum_score(fname) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let fname = "data/input";
    // let fname = "data/test_input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}", (end - start).as_millis());
}
