use std::env;
use std::fs;
use std::iter::repeat_n;

fn rolls_to_remove(grid: &mut[&mut [char]]) -> Option<usize> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut removed = 0;
    for row in 1..(n_rows - 1) {
        for col in 1..(n_cols - 1) {

            let mut count = 0;

            for i in [-1isize, 0, 1] {
                for j in [-1isize, 0, 1] {
                    if i != 0 || j != 0 {
                        let k = row as isize + i;
                        let l = col as isize + j;

                        if grid[k as usize][l as usize] == '@' {
                            count += 1;
                        }

                    }
                }
            }

            if count < 4 && grid[row][col] == '@' {
                removed += 1;
                grid[row][col] = '.';
            }
        }
    }

    if removed > 0 {
        return Some(removed)
    }
    else {
        return None;
    }
}

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(0, '.');
            chars.push('.');

            return chars;
        })
        .collect();

    let n_cols = grid[0].len();
    let empty_row: Vec<char> = repeat_n('.', n_cols).collect();

    grid.insert(0, empty_row.clone());
    grid.push(empty_row);

    let mut total = 0;

    let mut rows: Vec<&mut [char]> = grid.iter_mut().map(|row| row.as_mut_slice()).collect();
    while let Some(n) = rolls_to_remove(&mut rows) {
        total += n;
    }

    println!("{}", total);
}
