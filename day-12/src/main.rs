use std::fs;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let fname = "data/test_input_1";
        let result = solve_part_one(fname);
        assert_eq!(result, 140);
    }

    #[test]
    fn test_part_one_2() {
        let fname = "data/test_input_2";
        let result = solve_part_one(fname);
        assert_eq!(result, 772);
    }

    #[test]
    fn test_part_one_3() {
        let fname = "data/test_input_3";
        let result = solve_part_one(fname);
        assert_eq!(result, 1930);
    }
}

#[derive(Clone, Debug)]
struct Plot {
    x: i32,
    y: i32,
    plant: char,
}

impl Plot {
    fn was_visited(&self, visited: &Visited) -> bool {
        return visited.map[self.y as usize][self.x as usize];
    }

    fn get_neighbors(&self, garden: &Garden) -> Vec<Plot> {
        // Return a vec neighbors of the same plant type.
        let nrows = garden.plants.len() as i32;
        let ncols = garden.plants[0].len() as i32;
        let mut neighbors = vec![];
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if (self.x == 0 && *dx < 0) | (self.x == ncols - 1 && *dx > 0) {
                continue;
            }
            if (self.y == 0 && *dy < 0) | (self.y == nrows - 1 && *dy > 0) {
                continue;
            }
            let neighbor = Plot {
                x: self.x + dx,
                y: self.y + dy,
                plant: garden.get_plant(self.x + dx, self.y + dy),
            };
            if neighbor.plant == self.plant {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}

struct Visited {
    map: Vec<Vec<bool>>,
}

impl Visited {
    fn new(garden: &Garden) -> Self {
        let mut map = vec![];
        for row in garden.plants.iter() {
            map.push((0..row.len()).map(|_| false).collect());
        }
        Self { map }
    }

    fn visit(&mut self, plot: &Plot) {
        self.map[plot.y as usize][plot.x as usize] = true;
    }
}

struct Garden {
    plants: Vec<Vec<char>>,
}

impl Garden {
    pub fn new(plants: Vec<Vec<char>>) -> Self {
        Self { plants }
    }

    pub fn get_area_and_perimeter(&self, plot: Plot, visited: &mut Visited) -> (u32, u32) {
        // Compute area and perimeter of a region of plots of same type
        let (mut area, mut perimeter) = (0, 0);
        let mut stack: Vec<Plot> = vec![plot];
        while !stack.is_empty() {
            // Pop last element in stack
            let plot = stack.pop().unwrap();
            // If we already visited this neighbor, just continue
            if plot.was_visited(visited) {
                continue;
            };
            // Get neighbors of same type of plant
            let neighbors = plot.get_neighbors(self);
            // Increase area on one, and perimeter as (4 - number of neighbors)
            area += 1;
            perimeter += 4 - neighbors.len() as u32;
            // Mark this plot as visited
            visited.visit(&plot);
            // Add unvisited neighbors to the stack
            let unvisited_neighbors: Vec<Plot> = neighbors
                .iter()
                .filter(|n| !n.was_visited(visited))
                .map(|n| n.clone())
                .collect();
            stack.extend(unvisited_neighbors);
        }
        (area, perimeter)
    }

    fn get_plant(&self, x: i32, y: i32) -> char {
        return self.plants[y as usize][x as usize];
    }

    pub fn get_plot(&self, x: i32, y: i32) -> Plot {
        Plot {
            x,
            y,
            plant: self.get_plant(x, y),
        }
    }
}

fn read_file(fname: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let mut plants = vec![];
    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        plants.push(row);
    }
    return plants;
}

fn get_total_price(garden: &Garden) -> u32 {
    // Returns price of all regions in the garden
    let mut visited = Visited::new(&garden);
    let nrows = garden.plants.len();
    let ncols = garden.plants[0].len();
    let mut price = 0;
    // use y for rows (vertical axis) and x for cols (horizontal)
    for y in 0..nrows {
        for x in 0..ncols {
            let plot = garden.get_plot(x as i32, y as i32);
            if plot.was_visited(&visited) {
                continue;
            };
            let (area, perimeter) = garden.get_area_and_perimeter(plot, &mut visited);
            price += area * perimeter;
        }
    }
    price
}

fn solve_part_one(fname: &str) -> u32 {
    let plants = read_file(fname);
    let garden = Garden::new(plants);
    let total_price = get_total_price(&garden);
    total_price
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());
}
