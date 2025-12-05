use std::env;
use std::fs;

fn bank(digits: &[u32]) -> u32 {
    let len = digits.len();

    let max = digits[..len-1].into_iter().max().unwrap();
    let arg_max = digits.into_iter().position(|x| x == max).unwrap();

    let sec_max = digits[arg_max + 1..].into_iter().max().unwrap();

    return max * 10 + sec_max;
}

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let lines = file_contents.lines().map(|line| {
        let char_vec = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let x = bank(&char_vec);
        return x;
    });


    println!("{}", lines.sum::<u32>());
}