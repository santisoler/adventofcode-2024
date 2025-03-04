use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 36);
    }
    #[test]
    fn test_part_two() {
        let fname = "data/test_input";
        let result = solve_part_two(fname);
        assert_eq!(result, 81);
    }
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct Topo {
    map: Vec<Vec<u32>>,
    nrows: usize,
    ncols: usize,
}

impl Topo {
    pub fn get(&self, position: &Position) -> &u32 {
        return &self.map[position.y][position.x];
    }

    pub fn get_trailheads(&self) -> Vec<Position> {
        let mut trailheads: Vec<Position> = vec![];
        for (i, row) in self.map.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value == 0 {
                    trailheads.push(Position { x: j, y: i })
                }
            }
        }
        trailheads
    }

    pub fn get_trailhead_score(&self, position: &Position, summits: &mut Vec<Position>) -> u32 {
        // count how many summits can be reached from this trailhead
        if *self.get(position) == 9 && !summits.contains(&position) {
            summits.push(position.clone());
            return 1;
        }
        let neighbors = self.get_trail_neighbours(position);
        let result = neighbors
            .iter()
            .map(|n| self.get_trailhead_score(&n, summits))
            .sum();
        return result;
    }

    pub fn get_trailhead_rating(&self, position: &Position) -> u32 {
        // count how many trails can be followed from this trailhead
        if *self.get(position) == 9 {
            return 1;
        }
        let neighbors = self.get_trail_neighbours(position);
        let result = neighbors
            .iter()
            .map(|n| self.get_trailhead_rating(&n))
            .sum();
        return result;
    }

    fn is_delta_inside(&self, position: &Position, delta_x: i32, delta_y: i32) -> bool {
        if position.x == 0 && delta_x < 0 {
            return false;
        }
        if position.x == self.ncols - 1 && delta_x > 0 {
            return false;
        }
        if position.y == 0 && delta_y < 0 {
            return false;
        }
        if position.y == self.nrows - 1 && delta_y > 0 {
            return false;
        }
        return true;
    }

    fn get_trail_neighbours(&self, position: &Position) -> Vec<Position> {
        let height = self.get(position);
        let mut neighbors = vec![];
        for (dx, dy) in DELTAS {
            if !self.is_delta_inside(position, dx, dy) {
                continue;
            };
            let n = Position {
                x: (position.x as i32 + dx) as usize,
                y: (position.y as i32 + dy) as usize,
            };
            if *self.get(&n) == height + 1 {
                neighbors.push(n);
            }
        }
        return neighbors;
    }
}

fn read_file(fname: &str) -> Topo {
    let content = fs::read_to_string(fname).expect("Couldn't read file");
    let map = {
        let mut map: Vec<Vec<u32>> = vec![];
        for line in content.lines() {
            let row = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            map.push(row);
        }
        map
    };
    let nrows = map.len();
    let ncols = map[0].len();
    Topo { map, nrows, ncols }
}

fn solve_part_one(fname: &str) -> u32 {
    let topo = read_file(fname);
    let trailheads = topo.get_trailheads();
    trailheads
        .iter()
        .map(|t| {
            let mut summits: Vec<Position> = vec![];
            topo.get_trailhead_score(t, &mut summits)
        })
        .sum()
}

fn solve_part_two(fname: &str) -> u32 {
    let topo = read_file(fname);
    let trailheads = topo.get_trailheads();
    trailheads
        .iter()
        .map(|t| topo.get_trailhead_rating(t))
        .sum()
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = solve_part_two(fname);
    println!("Solution to part one: {result}");
}
