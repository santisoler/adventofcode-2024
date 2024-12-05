use std::fs;

fn find_xmas(soup: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let n = soup.len();
    let mut result: i32 = 0;
    // col
    if i < n - 3 {
        let word: String = (0..4).map(|delta| soup[i + delta][j]).collect();
        if word.eq("XMAS") {
            result += 1
        }
    }
    if i >= 3 {
        let word: String = (0..4).map(|delta| soup[i - delta][j]).collect();
        if word.eq("XMAS") {
            result += 1
        };
    }
    // row
    if j < n - 3 {
        let word: String = (0..4).map(|delta| soup[i][j + delta]).collect();
        if word.eq("XMAS") {
            result += 1
        }
    }
    if j >= 3 {
        let word: String = (0..4).map(|delta| soup[i][j - delta]).collect();
        if word.eq("XMAS") {
            result += 1
        }
    }
    // diagonals
    if i < n - 3 && j < n - 3 {
        let diag: String = (0..4).map(|delta| soup[i + delta][j + delta]).collect();
        if diag.eq("XMAS") {
            result += 1
        }
    }
    if i >= 3 && j < n - 3 {
        let diag: String = (0..4).map(|delta| soup[i - delta][j + delta]).collect();
        if diag.eq("XMAS") {
            result += 1
        }
    }
    if i >= 3 && j >= 3 {
        let diag: String = (0..4).map(|delta| soup[i - delta][j - delta]).collect();
        if diag.eq("XMAS") {
            result += 1
        }
    }
    if i < n - 3 && j >= 3 {
        let diag: String = (0..4).map(|delta| soup[i + delta][j - delta]).collect();
        if diag.eq("XMAS") {
            result += 1
        }
    }
    return result;
}

pub fn solve_part1(fname: &str) -> i32 {
    let content = fs::read_to_string(&fname).expect("Couldn't read");
    let soup: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let n = soup.len();
    let mut counts = 0;
    for i in 0..n {
        for j in 0..n {
            if soup[i][j] == 'X' {
                counts += find_xmas(&soup, i, j);
            };
        }
    }
    counts
}
