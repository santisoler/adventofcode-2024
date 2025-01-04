use std::fs;

type Point = (usize, usize);
type Times = Grid<Option<u32>>;
type Map = Grid<char>;

const DELTAS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const SIGNS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

struct Grid<T>
where
    T: Copy + Eq + PartialEq,
{
    grid: Vec<Vec<T>>,
}

impl<T: Copy + Eq + PartialEq> Grid<T> {
    fn new(nrows: usize, ncols: usize, fill_value: T) -> Self {
        let map = vec![vec![fill_value; ncols]; nrows];
        Self { grid: map }
    }
    fn get(&self, point: &Point) -> T {
        self.grid[point.1][point.0]
    }

    fn write(&mut self, point: &Point, value: T) {
        self.grid[point.1][point.0] = value;
    }

    fn nrows(&self) -> usize {
        self.grid.len()
    }

    fn ncols(&self) -> usize {
        self.grid[0].len()
    }

    fn find(&self, value: T) -> Option<Point> {
        for (y, row) in self.grid.iter().enumerate() {
            match row.iter().position(|c| *c == value) {
                Some(x) => return Some((x, y)),
                None => (),
            }
        }
        None
    }

    fn get_points_at_distance(&self, point: &Point, distance: u32) -> Vec<Point> {
        let mut points = vec![];
        let mut deltas: Vec<(i32, i32)> = vec![];
        deltas.extend(vec![
            (distance as i32, 0),
            (0, distance as i32),
            (-(distance as i32), 0),
            (0, -(distance as i32)),
        ]);
        for (sign_x, sign_y) in SIGNS.iter() {
            for d in (1..distance).into_iter().map(|d| d as i32) {
                deltas.push((sign_x * (distance as i32 - d), sign_y * d));
            }
        }
        for (dx, dy) in deltas.iter() {
            if (point.0 as i32 + *dx) < 0 || (point.0 as i32 + *dx > self.ncols() as i32 - 1) {
                continue;
            }
            if (point.1 as i32 + *dy) < 0 || (point.1 as i32 + *dy > self.ncols() as i32 - 1) {
                continue;
            }
            points.push((
                (point.0 as i32 + *dx) as usize,
                (point.1 as i32 + *dy) as usize,
            ));
        }
        points
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

impl Grid<char> {
    fn new_from(fname: &str) -> Self {
        let content = fs::read_to_string(fname).unwrap();
        let mut map: Vec<Vec<char>> = vec![];
        for line in content.lines() {
            let row: Vec<char> = line.chars().collect();
            map.push(row);
        }
        Self { grid: map }
    }

    /// Return a vec with the location of points of the path (in no order)
    fn get_path(&self) -> Vec<Point> {
        let mut path = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if *element != '#' {
                    path.push((x, y))
                }
            }
        }
        path
    }

    /// Calculate the time it takes to reach each point of the path.
    ///
    /// Return a `Map<Option<32>>` with the time it takes to reach each one of the points in the
    /// map. If the point is a wall, then its time will be `None`.
    fn get_times(&self) -> Times {
        let (nrows, ncols) = (self.nrows(), self.ncols());
        let mut times = Grid::new(nrows, ncols, None);
        let mut visited = Grid::new(nrows, ncols, false);

        // Mark S as the current point and assign a time zero to it
        let mut time = 0;
        let mut current = self.find('S').unwrap();
        times.write(&current, Some(time));

        let end = self.find('E').unwrap();
        while current != end {
            // Mark current point as visited
            visited.write(&current, true);
            // Look for the next point in the path
            let path_neighbors: Vec<Point> = self
                .get_neighbors(&current)
                .into_iter()
                .filter(|n| self.get(n) != '#')
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
        times
    }
}

fn count_cheats(map: &Map, times: &Times, threshold: u32, max_cheat_time: u32) -> u32 {
    let path = map.get_path();
    path.into_iter()
        .map(|point| count_cheats_from(point, map, times, threshold, max_cheat_time))
        .sum()
}

fn count_cheats_from(
    point: Point,
    map: &Map,
    times: &Times,
    threshold: u32,
    max_cheat_time: u32,
) -> u32 {
    let mut n_cheats = 0;
    let time = times.get(&point).unwrap();
    for distance in 2..=max_cheat_time {
        let equidistants = map.get_points_at_distance(&point, distance);
        for other in equidistants.iter() {
            if let Some(other_time) = times.get(&other) {
                let saved_time = other_time as i32 - (time + distance) as i32;
                if saved_time >= threshold as i32 {
                    n_cheats += 1;
                }
            }
        }
    }
    n_cheats
}

fn solve_part_one(fname: &str) -> u32 {
    let threshold = 100;
    let max_cheat_time = 2;
    let map = Grid::new_from(fname);
    let times = map.get_times();
    let n_cheats = count_cheats(&map, &times, threshold, max_cheat_time);
    n_cheats
}

fn solve_part_two(fname: &str) -> u32 {
    let threshold = 100;
    let max_cheat_time = 20;
    let map = Grid::new_from(fname);
    let times = map.get_times();
    let n_cheats = count_cheats(&map, &times, threshold, max_cheat_time);
    n_cheats
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");
}
