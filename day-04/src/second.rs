use std::fs;

fn is_cross(soup: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Row-column
    let (left, right) = (soup[i - 1][j], soup[i + 1][j]);
    let (bottom, top) = (soup[i][j - 1], soup[i][j + 1]);
    if ((left == 'M' && right == 'S') || (left == 'S' && right == 'M'))
        && ((bottom == 'M' && top == 'S') || (bottom == 'S' && top == 'M'))
    {
        return true;
    };
    // Diagonals
    let (bottom_left, top_right) = (soup[i - 1][j - 1], soup[i + 1][j + 1]);
    let (bottom_right, top_left) = (soup[i + 1][j - 1], soup[i - 1][j + 1]);
    if ((bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M'))
        && ((bottom_right == 'M' && top_left == 'S') || (bottom_right == 'S' && top_left == 'M'))
    {
        return true;
    };
    return false;
}

pub fn solve_part2(fname: &str) -> i32 {
    let content = fs::read_to_string(&fname).expect("Couldn't read");
    let soup: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let n = soup.len();
    let mut counts = 0;
    // Avoid searching for 'A' on the edges: they cannot be the center of any cross
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            if soup[i][j] == 'A' {
                counts += is_cross(&soup, i, j) as i32;
            };
        }
    }
    counts
}
