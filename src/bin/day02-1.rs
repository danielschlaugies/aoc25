use std::env;
use std::fs;

fn number_of_digits(mut x: u64) -> usize {
    let mut r = 0;
    while x > 0 {
        x /= 10;
        r += 1;
    }

    return r;
}

fn double(x: u64) -> u64 {
    let n_digits = number_of_digits(x);

    return x + x * 10u64.pow((n_digits).try_into().unwrap());
}

fn count_palindromes(low: u64, high: u64) -> u64 {
    let n_digits_low = number_of_digits(low);
    let n_digits_high = number_of_digits(high);

    let start = low / 10_u64.pow(((n_digits_low + 1) / 2).try_into().unwrap());
    let end = high / 10_u64.pow((n_digits_high / 2).try_into().unwrap());

    let mut total = 0;

    for i in start..=end {
        // TODO
        let d = double(i);
        if d >= low && d <= high {
            total += d;
        }
    }

    return total;
}

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let ranges: Vec<u64> = file_contents.split(",").map(|x| {
        let mut y = x.trim().split("-");
        let a = y.next().unwrap().parse::<u64>().unwrap();
        let b = y.next().unwrap().parse::<u64>().unwrap();

        return count_palindromes(a, b);
    }).collect();
    
    println!("{:?}", ranges.iter().sum::<u64>());
}