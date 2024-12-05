mod first;
mod second;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = first::solve_part1(fname);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let fname = "data/test_input";
        let result = second::solve_part2(fname);
        assert_eq!(result, 9);
    }
}

fn main() {
    let fname = "data/input";
    let result = first::solve_part1(&fname);
    println!("Solution to part 1: {result}");
    let result = second::solve_part2(&fname);
    println!("Solution to part 2: {result}");
}
