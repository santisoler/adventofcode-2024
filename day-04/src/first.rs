use std::fs;

fn count_xmas(string: &str) -> i32 {
    let mut counts = 0;
    counts += string.matches("XMAS").count();
    counts += string.matches("SAMX").count();
    counts as i32
}

fn get_columns(soup: &Vec<Vec<char>>) -> Vec<String> {
    let m = soup[0].len();
    let columns: Vec<String> = {
        let mut columns = Vec::new();
        for j in 0..m {
            let column: String = soup.iter().map(|row| &row[j]).collect();
            columns.push(column);
        }
        columns
    };
    return columns;
}

fn get_diagonal(soup: &Vec<Vec<char>>, index: i32) -> String {
    let mut diagonal: Vec<char> = Vec::new();
    let right_side = index >= 0;
    let index = index.abs() as usize;
    for i in 0..soup.len() - index {
        if right_side {
            diagonal.push(soup[i][i + index]);
        } else {
            diagonal.push(soup[i + index][i]);
        };
    }
    diagonal.iter().collect()
}

fn get_anti_diagonal(soup: &Vec<Vec<char>>, index: i32) -> String {
    let mut anti_diagonal: Vec<char> = Vec::new();
    let right_side = index >= 0;
    let index = index.abs() as usize;
    let n = soup.len();
    for i in 0..n - index {
        if right_side {
            anti_diagonal.push(soup[i + index][n - 1 - i]);
        } else {
            anti_diagonal.push(soup[i][n - 1 - index - i]);
        };
    }
    anti_diagonal.iter().collect()
}

fn get_diagonals(soup: &Vec<Vec<char>>) -> Vec<String> {
    let mut diagonals = Vec::new();
    let n_diagonals: i32 = soup.len() as i32 - 1;
    let mut i = -n_diagonals;
    while i <= n_diagonals {
        diagonals.push(get_diagonal(&soup, i));
        i += 1;
    }
    return diagonals;
}

fn get_anti_diagonals(soup: &Vec<Vec<char>>) -> Vec<String> {
    let mut anti_diagonals = Vec::new();
    let n_diagonals: i32 = soup.len() as i32 - 1;
    let mut i = -n_diagonals;
    while i <= n_diagonals {
        anti_diagonals.push(get_anti_diagonal(&soup, i));
        i += 1;
    }
    return anti_diagonals;
}

pub fn solve_part1(fname: &str) -> i32 {
    let content = fs::read_to_string(&fname).expect("Couldn't read");
    let soup: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut counts = 0;
    let rows = soup
        .iter()
        .map(|row| row.into_iter().collect::<String>())
        .into_iter();
    counts += rows.map(|s| count_xmas(&s)).sum::<i32>();

    let columns = get_columns(&soup);
    counts += columns.iter().map(|s| count_xmas(&s)).sum::<i32>();

    let diagonals = get_diagonals(&soup);
    counts += diagonals.iter().map(|s| count_xmas(&s)).sum::<i32>();

    let anti_diagonals = get_anti_diagonals(&soup);
    counts += anti_diagonals.iter().map(|s| count_xmas(&s)).sum::<i32>();

    counts
}
