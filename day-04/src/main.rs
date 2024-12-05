mod first;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = first::solve_part1(fname);
        assert_eq!(result, 18);
    }
}

fn main() {
    let fname = "data/input";
    let result = first::solve_part1(&fname);
    println!("Solution to part 1: {result}");
}
