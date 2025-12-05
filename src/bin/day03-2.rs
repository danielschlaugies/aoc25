use std::env;
use std::fs;

// input:
// - digits array of u32
// - r number of remainaing digits


// output: 
// largest number in digits, such that 

fn bank(digits: &[u64], r: usize) -> (u64, &[u64]) {
    let len = digits.len(); 

    let slice = &digits[..len - r];
    let max = slice.into_iter().max().unwrap();
    let argmax = digits.into_iter().position(|x| x == max).unwrap();


    return (*max, &digits[argmax + 1..]);
}

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let lines = file_contents.lines().map(|line| {
        let char_vec = line.chars().map(|c| u64::from(c.to_digit(10).unwrap())).collect::<Vec<u64>>();

        let mut digits: &[u64] = &char_vec;
        let mut s = 0;
        let mut r  = 12;

        for _ in 0..12 {
            r -= 1;
            let (max, remaining_digits) = bank(digits, r);
            digits = remaining_digits;
            s *= 10;
            s += max;
        }

        return s;
    });


    println!("{}", lines.sum::<u64>());
}