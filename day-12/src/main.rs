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

    #[test]
    fn test_part_two_1() {
        let fname = "data/test_input_1";
        let result = solve_part_two(fname);
        assert_eq!(result, 80);
    }

    #[test]
    fn test_part_two_2() {
        let fname = "data/test_input_2";
        let result = solve_part_two(fname);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_part_two_3() {
        let fname = "data/test_input_3";
        let result = solve_part_two(fname);
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_part_two_4() {
        let fname = "data/test_input_4";
        let result = solve_part_two(fname);
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part_two_5() {
        let fname = "data/test_input_5";
        let result = solve_part_two(fname);
        assert_eq!(result, 368);
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

    fn count_convex_corners(&self, neighbors: &Vec<Plot>) -> u32 {
        // Count how many convex corners the plot has
        let n_corners = match neighbors.len() {
            4 => 0,
            3 => 0,
            2 => {
                let n1 = &neighbors[0];
                let n2 = &neighbors[1];
                // Check if the neighbors are inline or not
                if (n1.x == n2.x) | (n1.y == n2.y) {
                    0 // inline neighbors (no corners)
                } else {
                    1 // not inline neighbors, one corner
                }
            }
            1 => 2, // it's a U plot, has two corners
            0 => 4, // no neighbors, it has 4 corners
            _ => panic!("more than 4 neighbors were found"),
        };
        n_corners
    }

    fn count_concave_corners(&self, garden: &Garden) -> u32 {
        // Count how many concave corners the plot has
        //
        // In order to count a concave corner only on a single one of its side plots,
        // we are going to run with dx > 0.
        let nrows = garden.plants.len() as i32;
        let ncols = garden.plants[0].len() as i32;
        let mut n_corners = 0;
        let dx = 1;
        for dy in [-1, 1].iter() {
            if self.x == ncols - 1 {
                continue;
            }
            if (self.y == 0 && *dy < 0) | (self.y == nrows - 1 && *dy > 0) {
                continue;
            }
            let self_plant = garden.get_plant(self.x, self.y);
            let neighbor_diagonal = garden.get_plant(self.x + dx, self.y + dy);
            let neighbor_right = garden.get_plant(self.x + dx, self.y);
            if (self_plant == neighbor_diagonal) && (self_plant != neighbor_right) {
                n_corners += 1
            }
            let neighbor_vertical = garden.get_plant(self.x, self.y + dy);
            if (self_plant == neighbor_diagonal) && (self_plant != neighbor_vertical) {
                n_corners += 1
            }
        }
        n_corners
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

    pub fn get_area_and_sides(&self, plot: Plot, visited: &mut Visited) -> (u32, u32) {
        // Compute area and number of sides of a region of plots of same type
        let (mut area, mut sides) = (0, 0);
        let mut stack: Vec<Plot> = vec![plot];
        while !stack.is_empty() {
            // Pop last element in stack
            let plot = stack.pop().unwrap();
            // If we already visited this neighbor, just continue
            if plot.was_visited(visited) {
                continue;
            };
            // Get neighbors of same type of plant
            let neighbors = plot.get_neighbors(&self);
            // Increase area on one, and perimeter as (4 - number of neighbors)
            area += 1;
            // Count new corners
            let convex_corners = plot.count_convex_corners(&neighbors);
            let concave_corners = plot.count_concave_corners(self);
            sides += convex_corners + concave_corners;
            // corners += plot.count_convex_corners(&neighbors) + plot.count_concave_corners(self);
            println!(
                "{:?} convex: {convex_corners} concave: {concave_corners}",
                plot
            );
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
        (area, sides)
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

fn get_total_price_w_discount(garden: &Garden) -> u32 {
    // Returns price of all regions in the garden (counting sides instead perimeter)
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
            println!("{:?}", plot);
            let (area, sides) = garden.get_area_and_sides(plot, &mut visited);
            println!("    area: {area}, sides: {sides}");
            price += area * sides;
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

fn solve_part_two(fname: &str) -> u32 {
    let plants = read_file(fname);
    let garden = Garden::new(plants);
    let total_price = get_total_price_w_discount(&garden);
    total_price
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    // let result = solve_part_two(fname);
    // println!("Solution to part one: {result}");

    let fname = "data/test_input_4";
    let result = solve_part_two(fname);
    println!("Solution to part one: {result}");
}
