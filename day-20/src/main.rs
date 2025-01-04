use std::collections::{HashMap, HashSet};
use std::fs;

type Point = (usize, usize);
type Path = Vec<Point>;

const DELTAS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

struct Map<T>
where
    T: Copy + Eq + PartialEq,
{
    map: Vec<Vec<T>>,
}

impl<T: Copy + Eq + PartialEq> Map<T> {
    fn new(nrows: usize, ncols: usize, fill_value: T) -> Self {
        let map = vec![vec![fill_value; ncols]; nrows];
        Self { map }
    }
    fn get(&self, point: &Point) -> T {
        self.map[point.1][point.0]
    }

    fn write(&mut self, point: &Point, value: T) {
        self.map[point.1][point.0] = value;
    }

    fn nrows(&self) -> usize {
        self.map.len()
    }

    fn ncols(&self) -> usize {
        self.map[0].len()
    }

    fn find(&self, value: T) -> Option<Point> {
        for (y, row) in self.map.iter().enumerate() {
            match row.iter().position(|c| *c == value) {
                Some(x) => return Some((x, y)),
                None => (),
            }
        }
        None
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        for (dx, dy) in DELTAS.iter() {
            if (point.0 == 0 && *dx < 0) || (point.0 == self.ncols() - 1 && *dx > 0) {
                continue;
            }
            if (point.1 == 0 && *dy < 0) || (point.1 == self.nrows() - 1 && *dy > 0) {
                continue;
            }
            neighbors.push((
                (point.0 as i32 + *dx) as usize,
                (point.1 as i32 + *dy) as usize,
            ));
        }
        neighbors
    }
}

impl Map<char> {
    fn is_wall(&self, point: &Point) -> bool {
        self.map[point.1][point.0] == '#'
    }
}

fn parse_file(fname: &str) -> Map<char> {
    let content = fs::read_to_string(fname).unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    Map { map }
}

/// Obtain the path from S to E.
/// Return a Path with each point in the path, and a Map<Option<32>> with the time it takes to
/// reach each one of the points in the map. If the point is a wall, then its time will be None.
fn get_path_and_times(map: &Map<char>) -> (Path, Map<Option<u32>>) {
    let mut path: Path = vec![];
    let (nrows, ncols) = (map.nrows(), map.ncols());
    let mut times = Map::new(nrows, ncols, None);
    let mut visited = Map::new(nrows, ncols, false);

    // Mark S as the current point and assign a time zero to it
    let mut time = 0;
    let mut current = map.find('S').unwrap();
    times.write(&current, Some(time));

    let end = map.find('E').unwrap();
    while current != end {
        // Add current point to the path and mark it as visited
        path.push(current);
        visited.write(&current, true);
        // Look for the next point in the path
        let path_neighbors: Vec<Point> = map
            .get_neighbors(&current)
            .into_iter()
            .filter(|n| map.get(n) != '#')
            .filter(|n| !visited.get(n))
            .collect();
        if path_neighbors.len() != 1 {
            panic!("Found invalid neighbors: {:?}", path_neighbors);
        };
        // Update current point and write down the time it takes to get to it
        current = path_neighbors[0];
        time += 1;
        times.write(&current, Some(time));
    }
    (path, times)
}

/// Find cheats that would save a time at least of the given threshold
fn count_cheats(map: &Map<char>, path: &Path, times: &Map<Option<u32>>, threshold: u32) -> u32 {
    let mut n_cheats = 0;
    for (time, point) in path.iter().enumerate().map(|(t, p)| (t as u32, p)) {
        // Find neighboring walls
        let wall_neighbors: Vec<Point> = map
            .get_neighbors(point)
            .into_iter()
            .filter(|n| map.get(n) == '#')
            .collect();
        for wall in wall_neighbors.iter() {
            for neighbor in map.get_neighbors(wall).iter() {
                if let Some(neighbor_time) = times.get(neighbor) {
                    // Need to add 2 to the threshold because it takes 2 picoseconds to get from
                    // the first path point through the cheat to the next path point.
                    if neighbor_time > time && (neighbor_time - time) >= threshold + 2 {
                        n_cheats += 1;
                    }
                }
            }
        }
    }
    n_cheats
}

/// Count how many cheats can be used from a given star point.
fn count_cheats_from(
    point: &Point,
    map: &Map<char>,
    times: &Map<Option<u32>>,
    max_cheat_time: u32,
) -> Vec<u32> {
    // Define the time of the starting point
    let start_time = times.get(&point).unwrap();
    // Define a vec where we are going to store the times that can be saved using cheats that start
    // from this point.
    let mut times_saved = HashSet::<u32>::new();
    // Define a stack of walls and the cheat time it takes to get to them
    let mut walls: Vec<(Point, u32)> = vec![];
    let mut visited = Map::new(map.nrows(), map.ncols(), false);
    // Initialize the stack with the wall neighbors of the current point
    let wall_neighbors = map
        .get_neighbors(&point)
        .into_iter()
        .filter(|n| map.is_wall(n))
        .collect::<Vec<Point>>();
    for n in wall_neighbors.into_iter() {
        walls.push((n, 1));
    }
    while !walls.is_empty() {
        // println!("walls len: {}", walls.len());
        let (wall, cheat_time) = walls.pop().unwrap();
        if cheat_time >= max_cheat_time {
            continue;
        }
        visited.write(&wall, true);

        for neighbor in map.get_neighbors(&wall) {
            if visited.get(&neighbor) {
                continue;
            };
            if map.is_wall(&neighbor) {
                walls.push((neighbor, cheat_time + 1))
            } else {
                let end_time = times.get(&neighbor).unwrap();
                let saved_time: i32 = end_time as i32 - start_time as i32 - (cheat_time + 1) as i32;
                // println!(
                //     "neighbor: {:?}, end_time: {end_time}, saved_time: {saved_time}",
                //     neighbor
                // );
                if saved_time > 0 {
                    times_saved.insert(saved_time as u32);
                }
            }
        }
    }
    // println!("times_saved: {:?}", times_saved);
    times_saved.into_iter().collect::<Vec<u32>>()
}

fn count_cheats_by_saved_time(
    path: &Path,
    map: &Map<char>,
    times: &Map<Option<u32>>,
    max_cheat_time: u32,
) -> HashMap<u32, u32> {
    let mut cheats = HashMap::<u32, u32>::new();
    for point in path.iter() {
        let times_saved = count_cheats_from(&point, map, times, max_cheat_time);
        for time_saved in times_saved.into_iter() {
            cheats
                .entry(time_saved)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }
    cheats
}

fn solve_part_one(fname: &str) -> u32 {
    let map = parse_file(fname);
    let (path, times) = get_path_and_times(&map);
    count_cheats(&map, &path, &times, 100)
}

fn solve_part_two(fname: &str) -> u32 {
    let max_cheat_time = 20;
    let threshold = 50;
    let map = parse_file(fname);
    let (path, times) = get_path_and_times(&map);
    let cheats = count_cheats_by_saved_time(&path, &map, &times, max_cheat_time);
    for (saved_time, count) in cheats.iter() {
        if *saved_time >= threshold {
            println!("{}: {}", saved_time, count)
        }
    }
    let n_cheats = cheats
        .iter()
        .filter(|(saved_time, _)| **saved_time >= threshold)
        .map(|(_, count)| count)
        .sum();
    n_cheats
}

fn main() {
    // let fname = "data/input";
    let fname = "data/test_input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");

    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");

    // ----------
    // let max_cheat_time = 20;
    // let threshold = 0;
    // let map = parse_file(fname);
    // let (_, times) = get_path_and_times(&map);
    // let point = map.find('S').unwrap();
    // let times_saved = count_cheats_from(&point, &map, &times, max_cheat_time);
    // println!("times_saved: {:?}", times_saved);
}
