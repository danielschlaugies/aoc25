use std::env;
use std::fs;

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut lines: Vec<Vec<&str>> = file_contents.lines().map(|line| line.split_whitespace().collect()).collect();

    let operations = lines.pop().unwrap();

    let numbers: Vec<Vec<u64>> = lines.iter().map(|line| {
        return line.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    }).collect();

    let mut total = 0;
    for col in 0..operations.len() {
        let operation = match operations[col] {
            "*" => |x: u64, y: u64| x * y,
            "+" => |x, y| x + y,
            _ => |_, _| 0
        };

        total += numbers.iter().map(|line| line[col]).reduce(operation).unwrap();
    }



    println!("{}", total);
}