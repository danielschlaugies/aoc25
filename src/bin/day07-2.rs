use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut grid: Vec<Vec<i64>> = file_contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    return match c {
                        '.' => 0,
                        '^' => -1,
                        //magic value
                        'S' => -42,
                        //should never happen
                        _ => -1337,
                    };
                })
                .collect()
        })
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for row in 0..(n_rows - 1) {
        for col in 0..n_cols {
            //starting point
            if grid[row][col] == -42 {
                grid[row + 1][col] = 1;
            } else if grid[row][col] > 0 {
                if grid[row + 1][col] >= 0 {
                    grid[row + 1][col] += grid[row][col];
                } else if grid[row + 1][col] == -1 {
                    grid[row + 1][col - 1] += grid[row][col];
                    grid[row + 1][col + 1] += grid[row][col];
                }
            }
        }
    }

    println!("{:?}", grid.last().unwrap().iter().sum::<i64>());
}
