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

fn solve_part_one(fname: &str) -> u32 {
    let map = parse_file(fname);
    let (path, times) = get_path_and_times(&map);
    count_cheats(&map, &path, &times, 100)
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
