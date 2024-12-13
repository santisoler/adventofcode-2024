mod first;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = first::solve_part_one(fname);
        assert_eq!(result, 1928);
    }
}

fn main() {
    let fname = "data/input";
    let result = first::solve_part_one(fname);
    println!("Solution to part one: {result}");
}
