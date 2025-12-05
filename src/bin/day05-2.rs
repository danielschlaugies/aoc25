use std::env;
use std::fs;
use std::collections::HashSet;
use std::cmp::{min, max};

struct IntervalManager {
    disjoint_intervals: HashSet<(u64, u64)>
}

impl IntervalManager {
    fn add_interval(&mut self, mut low: u64, mut high: u64) {

        let mut intervals_to_remove: Vec<(u64, u64)> = Vec::new();

        for (x, y) in &self.disjoint_intervals {
            //disjoint interval
            if (*y < low) || (*x > high) {
                //nothing to do
            }
            else {
                intervals_to_remove.push((*x, *y));
                if low <= *y {
                    low = min(*x, low);
                }
                if high >= *x {
                    high = max(*y, high);
                }
            }
        }

        // println!("intervals to remove: {:?}", intervals_to_remove);
        for interval in intervals_to_remove {
            self.disjoint_intervals.remove(&interval);
        }

        // println!("interval to insert: {:?}", (low, high));
        self.disjoint_intervals.insert((low, high));
    } 

    fn count(&self) -> u64 {
        return self.disjoint_intervals.iter().map(|(x, y)| y - x + 1).sum()
    }

    fn new() -> IntervalManager {
        return IntervalManager { disjoint_intervals: HashSet::new() }
    }
}

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

    let mut interval_manager = IntervalManager::new();
    
    for (x, y) in ranges {
        interval_manager.add_interval(x, y);

        // println!("{:?}", interval_manager.disjoint_intervals);
    }

    println!("{}", interval_manager.count());

}