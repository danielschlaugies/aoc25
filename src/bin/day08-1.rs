use core::f64;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Point {
    fn from(item: &str) -> Self {
        let mut splits = item.split(",");
        let x = splits.next().unwrap().parse::<i32>().unwrap();
        let y = splits.next().unwrap().parse::<i32>().unwrap();
        let z = splits.next().unwrap().parse::<i32>().unwrap();

        return Point { x, y, z };
    }
}

impl Point {
    fn distance(self, other: Point) -> f64 {

        let a = f64::from(self.x - other.x).powf(2.0);
        let b = f64::from(self.y - other.y).powf(2.0);
        let c = f64::from(self.z - other.z).powf(2.0);

        return f64::from(a + b + c).sqrt();
    }
}

fn main() {
    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let points: Vec<Point> = file_contents.lines().map(|l| Point::from(l)).collect();

    // println!("points: {:?}", points);

    let mut pairs: Vec<(Point, Point)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push((points[i], points[j]));
        }
    }

    pairs.sort_by(|a, b| {
        let d1 = a.0.distance(a.1);
        let d2 = b.0.distance(b.1);

        return f64::total_cmp(&d1, &d2);
    });

    let mut circles: Vec<HashSet<Point>> = Vec::new();

    for p in points {
        let mut h: HashSet<Point> = HashSet::new();
        h.insert(p);
        circles.push(h);
    }

    for i in 0..1000 {
        let (a, b) = pairs[i];

        let mut a_index = 10000;
        for k in 0..circles.len() {
            if circles[k].contains(&a) {
                a_index = k;
            }
        }

        let a_circle = circles[a_index].clone();
        circles.remove(a_index);

        let mut joined_b = false;
        for k in 0..circles.len() {
            if circles[k].contains(&b) {
                circles[k] = a_circle.union(&circles[k]).copied().collect();
                joined_b = true;
                break;
            }
        }

        if !joined_b {
            circles.push(a_circle);
        }


    }
   
    let mut lenghts: Vec<usize> = circles.iter().map(|h| h.len()).collect();
    lenghts.sort();
    lenghts.reverse();


    println!("{:?}", &lenghts[0..3].iter().product::<usize>());
}
