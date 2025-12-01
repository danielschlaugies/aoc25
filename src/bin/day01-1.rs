use std::env;
use std::fs;

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let n = 100;
    let mut dial: i32 = 50;

    let mut r = 0;
    for line in file_contents.lines() {
        let direction_char = line.chars().nth(0).unwrap();
        let direction = match direction_char {
            'L' => -1,
            'R' => 1,
            _ => 0
        };
        let numstring = &line[1..];
        let k = direction * numstring.parse::<i32>().unwrap();

        dial += k;
        dial %= n;

        if dial < 0 {
            dial += n;
        }

        if dial == 0 {
             r += 1;
        }
    }

    println!("{}", r);
}