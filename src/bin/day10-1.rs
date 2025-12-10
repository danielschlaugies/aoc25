use itertools::Itertools;
use std::cmp::min;
use std::env;
use std::fs;
use std::usize;

#[derive(Debug)]
struct Machine {
    target: u32,
    buttons: Vec<u32>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut s = value.split_whitespace();

        let tartget_arr: Vec<char> = s.next().unwrap().chars().collect();
        let mut target: u32 = 0;

        for i in 1..(tartget_arr.len() - 1) {
            if tartget_arr[i] == '#' {
                target |= 1 << (i - 1);
            }
        }

        let mut buttons: Vec<u32> = Vec::new();

        while let Some(button_str) = s.next() {
            if button_str.starts_with("(") {
                let mut button: u32 = 0;

                let honest_button_string = &button_str[1..(button_str.len() - 1)];
                let mut button_string_splits = honest_button_string.split(",");

                while let Some(charstring) = button_string_splits.next() {
                    let n: u32 = charstring.parse().unwrap();
                    button |= 1 << n;
                }

                buttons.push(button);
            }
        }

        return Machine { target, buttons };
    }
}

impl Machine {
    fn fewest_total_press(&self) -> usize {
        let mut min_total_press: usize = usize::MAX;

        for subset in self.buttons.iter().powerset() {
            let k = subset.len();
            if k < min_total_press {
                let mut n: u32 = 0;

                for x in subset {
                    n ^= x;
                }

                if n == self.target {
                    min_total_press = min(min_total_press, k);
                }
            }
        }

        return min_total_press;
    }
}

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let n: usize = file_contents
        .lines()
        .map(Machine::from)
        .map(|m| m.fewest_total_press())
        .sum();

    println!("{}", n);
}
