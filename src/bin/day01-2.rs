use std::env;
use std::fs;

struct Dial {
    state: i32,
    pointing_zero: usize,
}

impl Dial {
    fn right(&mut self) {
        self.state += 1;
        if self.state % 100 == 0 {
            self.state = 0;
            self.pointing_zero += 1;
        }
    }

    fn left(&mut self) {
        self.state -= 1;
        if self.state % 100 == 0 {
            self.state = 0;
            self.pointing_zero += 1;
        } else if self.state < 0 {
            self.state = 99;
        }
    }

    fn new() -> Self {
        return Dial {
            state: 50,
            pointing_zero: 0,
        };
    }
}

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();


    let mut dial = Dial::new();

    for line in file_contents.lines() {
        let direction_char = line.chars().nth(0).unwrap();
        let k = line[1..].parse::<i32>().unwrap();

        for _ in 0..k {
            match direction_char {
                'R' => dial.right(),
                'L' => dial.left(),
                _ => {}
            }
        }
    }

    println!("{}", dial.pointing_zero);
}