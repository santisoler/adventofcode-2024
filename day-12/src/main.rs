use std::fs;
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

    fn was_visited(&self, plot: &Plot) -> bool {
        return self.map[plot.y as usize][plot.x as usize];
    }
}

struct Garden {
    plants: Vec<Vec<char>>,
}

impl Garden {
    pub fn new(plants: Vec<Vec<char>>) -> Self {
        Self { plants }
    }

    pub fn get_area_and_perimeter(&self, plot: &Plot, visited: &mut Visited) -> (u32, u32) {
        let (area, perimeter) = self.walk(plot, visited);
        (area, perimeter)
    }

    fn walk(&self, plot: &Plot, visited: &mut Visited) -> (u32, u32) {
        // If this plot was visited by another walk path return 0 area and 0
        // perimeter to not count it twice.
        if visited.was_visited(plot) {
            return (0, 0);
        }
        // Walk the area of plots of the same plant type
        let neighbors = self.get_neighbors(plot);
        // Increase area on one, and perimeter as (4 - number of neighbors)
        let mut area = 1;
        let mut perimeter = 4 - neighbors.len() as u32;
        // Mark this plot as visited
        visited.visit(plot);
        // Run this recursively to visit all plots of the same plant type
        let unvisited_neighbors: Vec<Plot> = neighbors
            .iter()
            .filter(|n| !visited.was_visited(n))
            .map(|n| n.clone())
            .collect();
        if unvisited_neighbors.len() == 0 {
            return (area, perimeter);
        }
        for neighbor in unvisited_neighbors.iter() {
            let (area_i, perimeter_i) = self.walk(neighbor, visited);
            area += area_i;
            perimeter += perimeter_i;
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

    fn get_neighbors(&self, plot: &Plot) -> Vec<Plot> {
        // Return a vec neighbors of the same plant type.
        let nrows = self.plants.len() as i32;
        let ncols = self.plants[0].len() as i32;
        let mut neighbors = vec![];
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            if (plot.x == 0 && *dx < 0) | (plot.x == ncols - 1 && *dx > 0) {
                continue;
            }
            if (plot.y == 0 && *dy < 0) | (plot.y == nrows - 1 && *dy > 0) {
                continue;
            }
            let neighbor = Plot {
                x: plot.x + dx,
                y: plot.y + dy,
                plant: self.get_plant(plot.x + dx, plot.y + dy),
            };
            if neighbor.plant == plot.plant {
                neighbors.push(neighbor);
            }
        }
        neighbors
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
            if visited.was_visited(&plot) {
                continue;
            };
            let (area, perimeter) = garden.get_area_and_perimeter(&plot, &mut visited);
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
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
