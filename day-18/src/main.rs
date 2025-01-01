use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs;

// const MAP_SIZE: usize = 6 + 1;
const MAP_SIZE: usize = 70 + 1;
const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

/// Tile struct used in the BinaryHeap
struct PositionDist {
    point: (usize, usize),
    distance: u32,
}

impl Ord for PositionDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for PositionDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PositionDist {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PositionDist {}

// --------

struct Map<T> {
    map: [[T; MAP_SIZE]; MAP_SIZE],
}

impl<T> Map<T> {
    /// Create a new map filled with the same value
    fn new(fill: T) -> Self
    where
        T: Copy,
    {
        Self {
            map: [[fill; MAP_SIZE]; MAP_SIZE],
        }
    }

    /// Overwrite value to the map for a given point
    fn write(&mut self, point: &(usize, usize), value: T)
    where
        T: Copy,
    {
        self.map[point.1][point.0] = value;
    }

    /// Overwrite value to the map for a given point
    fn get(&self, point: &(usize, usize)) -> T
    where
        T: Copy,
    {
        self.map[point.1][point.0]
    }
}

fn get_neighbors(point: &(usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = (point.0, point.1);
    let mut neighbors = vec![];
    for (delta_x, delta_y) in DELTAS.iter() {
        if ((x == 0) && (*delta_x < 0)) || ((x == MAP_SIZE - 1) && (*delta_x > 0)) {
            continue;
        };
        if (y == 0 && *delta_y < 0) || ((y == MAP_SIZE - 1) && *delta_y > 0) {
            continue;
        };
        neighbors.push(((x as i32 + delta_x) as usize, (y as i32 + delta_y) as usize))
    }
    neighbors
}

fn read_file(fname: &str) -> Vec<(usize, usize)> {
    let content = fs::read_to_string(fname).unwrap();
    let mut points = vec![];
    for line in content.lines() {
        let mut coords = line.split(",");
        points.push((
            coords.next().unwrap().parse::<usize>().unwrap(),
            coords.next().unwrap().parse::<usize>().unwrap(),
        ))
    }
    points
}

fn get_minimum_distance(
    corrupted: &Map<bool>,
    start: (usize, usize),
    end: (usize, usize),
) -> Result<u32, &str> {
    let mut distances = Map::new(u32::MAX);
    let mut visited = Map::new(false);
    let mut queue = BinaryHeap::<Reverse<PositionDist>>::new();
    queue.push(Reverse(PositionDist {
        point: (start.0, start.1),
        distance: 0,
    }));
    distances.write(&start, 0);

    while !queue.is_empty() {
        let point = queue.pop().unwrap().0;
        if point.point == end {
            return Ok(point.distance);
        }
        visited.write(&point.point, true);

        for neighbor in get_neighbors(&point.point) {
            if visited.get(&neighbor) || corrupted.get(&neighbor) {
                continue;
            };
            let neighbor_distance = point.distance + 1;
            if neighbor_distance < distances.get(&neighbor) {
                distances.write(&neighbor, neighbor_distance);
                queue.push(Reverse(PositionDist {
                    point: neighbor,
                    distance: neighbor_distance,
                }));
            };
        }
    }
    Err("Couldn't find path to exit")
}

fn solve_part_one(fname: &str) -> u32 {
    let bytes = read_file(fname);
    let mut corrupted = Map::new(false);
    let start = (0, 0);
    let end = (MAP_SIZE - 1, MAP_SIZE - 1);
    // Make the first kilobyte fall (mark those positions as corrupted)
    for byte in bytes[0..1024].iter() {
        corrupted.write(byte, true)
    }
    match get_minimum_distance(&corrupted, start, end) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
