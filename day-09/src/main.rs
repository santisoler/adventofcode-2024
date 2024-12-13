mod first;
mod second;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = first::solve_part_one(fname);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_two() {
        let fname = "data/test_input";
        let result = second::solve_part_two(fname);
        assert_eq!(result, 2858);
    }
}

fn main() {
    let fname = "data/input";
    let result = first::solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = second::solve_part_two(fname);
    println!("Solution to part two: {result}");
}
