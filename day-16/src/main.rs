use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::time::Instant;

const DIRECTIONS: [Direction; 3] = [Direction::Forward, Direction::Right, Direction::Left];

struct Maze {
    map: Vec<Vec<char>>,
}

impl Maze {
    // Create new maze from input file
    fn new_from(fname: &str) -> Self {
        let content = fs::read_to_string(fname).unwrap();
        let mut map: Vec<Vec<char>> = vec![];
        for line in content.lines() {
            let row = line.chars().collect();
            map.push(row);
        }
        Self { map }
    }

    // Get start position
    fn get_start(&self) -> (usize, usize) {
        let j = self.map.len() - 2; // S is always in the previous to last row
        let i = self.map[j].iter().position(|c| *c == 'S').unwrap();
        (i, j)
    }

    // Get end position
    fn get_end(&self) -> (usize, usize) {
        let j = 1; // S is always in the second row
        let i = self.map[j].iter().position(|c| *c == 'E').unwrap();
        (i, j)
    }
}

enum Direction {
    Forward,
    Right,
    Left,
}

impl Direction {
    fn as_int(&self) -> i32 {
        match self {
            Direction::Forward => 0,
            Direction::Right => 1,
            Direction::Left => -1,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn rotate(&self, direction: &Direction) -> Self {
        self.from_int(self.as_int() + direction.as_int())
    }

    fn as_int(&self) -> i32 {
        match self {
            Orientation::North => 0,
            Orientation::East => 1,
            Orientation::South => 2,
            Orientation::West => 3,
        }
    }

    fn from_int(&self, integer: i32) -> Self {
        match integer.rem_euclid(4) {
            0 => Orientation::North,
            1 => Orientation::East,
            2 => Orientation::South,
            3 => Orientation::West,
            _ => panic!("Invalid integer {integer}"),
        }
    }
}

struct Tile {
    x: usize,
    y: usize,
    score: u32,
    orientation: Orientation,
}

// Need to implement the Ord trait for Tile so it can be used in the BinaryHeap.
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Tile {}

fn get_lowest_score(fname: &str) -> Result<u32, &str> {
    let maze = Maze::new_from(fname);

    let start = maze.get_start();
    let end = maze.get_end();

    // Define HashMaps for storing the scores and the visted tiles.
    // It's important tha these hashmaps also consider the orientation of the tiles, and not just
    // their positions. Two tiles with same position but different orientations should be treated
    // differently. It's like the maze in four folds: one for each direction.
    // Failing to do so would not result in the lowest score.
    let mut scores = HashMap::<(usize, usize, Orientation), u32>::new();
    let mut visited = HashMap::<(usize, usize, Orientation), bool>::new();

    let mut heap = BinaryHeap::<Reverse<Tile>>::new();
    let start_tile = Tile {
        x: start.0,
        y: start.1,
        score: 0,
        orientation: Orientation::East,
    };
    scores.insert(
        (start_tile.x, start_tile.y, start_tile.orientation),
        start_tile.score,
    );
    heap.push(Reverse(start_tile));

    while !heap.is_empty() {
        // Pop from heap (the tile with lowest score)
        let tile = heap.pop().unwrap().0;

        // Mark as visited
        visited
            .entry((tile.x, tile.y, tile.orientation))
            .and_modify(|v| *v = true)
            .or_insert(true);

        // If target tile, return score
        if (tile.x, tile.y) == end {
            return Ok(tile.score);
        }

        for direction in DIRECTIONS {
            // Get the neighboring tile
            let orientation = tile.orientation.rotate(&direction);
            let (x, y) = match orientation {
                Orientation::East => (tile.x + 1, tile.y),
                Orientation::West => (tile.x - 1, tile.y),
                Orientation::North => (tile.x, tile.y - 1),
                Orientation::South => (tile.x, tile.y + 1),
            };
            // Skip if neighbor is a wall or if it was already visited
            if maze.map[y][x] == '#' {
                continue;
            };
            if visited.contains_key(&(x, y, orientation)) {
                continue;
            }
            // Compute the score of the neighbor tile
            let score = match direction {
                Direction::Forward => tile.score + 1,
                Direction::Left | Direction::Right => tile.score + 1 + 1000,
            };
            // If we found a path that leads to the neighbor tile with a
            // lower score, add it to the heap.
            let key = (x, y, orientation);
            if !scores.contains_key(&key) || (score < *scores.get(&key).unwrap()) {
                let neighbor = Tile {
                    x,
                    y,
                    score,
                    orientation,
                };
                heap.push(Reverse(neighbor));
                scores
                    .entry(key)
                    .and_modify(|s| *s = score)
                    .or_insert(score);
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
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Estimated time: {}s", (end - start).as_secs_f64());
    println!("Estimated time: {}ms", (end - start).as_millis());
}
