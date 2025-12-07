use std::array::repeat;
use std::env;
use std::fs;
use std::iter::repeat_n;

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut grid: Vec<String> = file_contents.lines().map(|line| {
        return ".".to_owned() + line + ".";
    }).collect();

    let n_cols = grid[0].len();
    let empty_col: String = repeat_n('.', n_cols).collect();

    grid.insert(0, empty_col.clone());
    grid.push(empty_col);

    let n_rows = grid.len(); 

    let mut total = 0;
    for row in 1..(n_rows -1) {
        for col in 1..(n_cols -1) {

            let mut papers = 0;
            
            for i in [-1isize, 0, 1] {
                for j in [-1isize, 0, 1] {
                    if i != 0 || j != 0 {
                        let k = row as isize + i;
                        let l = col as isize + j;
                        if grid[k as usize].chars().nth(l as usize).unwrap() == '@' {
                            papers += 1;
                        }
                    }
                }
            }

            if papers < 4 && grid[row].chars().nth(col).unwrap() == '@' {
                total += 1;
            }
        }
    }

    println!("{}", total);
}