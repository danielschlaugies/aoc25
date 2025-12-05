use std::env;
use std::fs;

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let mut splits = file_contents.split("\n\n");

    let ranges = splits.next().unwrap().lines().map(|line| {
        let mut l = line.split("-");
        let low = l.next().unwrap().parse::<u64>().unwrap();
        let high = l.next().unwrap().parse::<u64>().unwrap();

        return (low, high);
    });


    let ids = splits.next().unwrap().lines().map(|line| {

        let n = line.parse::<u64>().unwrap();

        let mut in_interval = ranges.clone().map(|interval| {
            return interval.0 <= n && n <= interval.1 ;
        });

        return in_interval.any(|x| x);

    });

    // println!("{:?}", ids.collect::<Vec<bool>>());
    println!("{}", ids.filter(|b| *b).count());

}