use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut total_splits = 0;
    for row in 0..(n_rows - 1) {
        for col in 0..n_cols {
            let c = grid[row][col];

            match c {
                'S' => grid[row + 1][col] = '|',
                '|' => {
                    if grid[row + 1][col] == '.' {
                        grid[row + 1][col] = '|';
                    }
                    else if grid[row + 1][col] == '^' {
                        grid[row + 1][col - 1]  = '|';
                        grid[row + 1][col + 1] = '|';
                        total_splits += 1;
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}", total_splits);
}
