use std::cmp::max;
use std::env;
use std::fs;

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let points: Vec<(i64, i64)> = file_contents.lines().map(|line| {
        let mut split = line.split(',');
        let a: i64 = split.next().unwrap().parse().unwrap();
        let b: i64 = split.next().unwrap().parse().unwrap();

        return (a, b)
    }).collect();

    let mut max_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = points[i];
            let b = points[j];

            let x = max((a.0 - b.0 + 1).abs(), (b.0 - a.0 + 1).abs());
            let y = max((a.1 - b.1 + 1).abs(), (b.1 - a.1 + 1).abs());
            
            max_area = max(max_area, x * y);
        }
    }

    println!("{}", max_area);
}