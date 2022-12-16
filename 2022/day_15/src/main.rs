use std::cmp::max;
use std::str::FromStr;
use std::{env, fs};

use regex::Regex;

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon_distance: i64,
}

impl Sensor {
    fn covered_on_row(&self, y: i64) -> (i64, i64) {
        let dist = distance(self.pos, (self.pos.0, y));
        if dist <= self.beacon_distance {
            let diff = self.beacon_distance - dist;
            (self.pos.0 - diff, self.pos.0 + diff)
        } else {
            (0, 0)
        }
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let captures = regex.captures(s).unwrap();
        let pos = (
            captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let closest_beacon = (
            captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        );
        Ok(Sensor {
            pos,
            beacon_distance: distance(pos, closest_beacon),
        })
    }
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn covered_ranges(y: i64, sensors: &Vec<Sensor>) -> Vec<(i64, i64)> {
    let mut covered_ranges = sensors
        .iter()
        .map(|s| s.covered_on_row(y))
        .filter(|&(x1, x2)| x1 != 0 || x2 != 0)
        .collect::<Vec<(i64, i64)>>();
    covered_ranges.sort_by_key(|&(x1, _)| x1);

    covered_ranges
}

fn merge_ranges(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut covered = ranges[0].1 - ranges[0].0;
    let mut x_max = ranges[0].1;
    for &(x1, x2) in ranges.iter().skip(1) {
        if x1 > x_max {
            covered += x2 - x1;
        } else if x2 > x_max {
            covered += x2 - x_max;
        }

        x_max = max(x_max, x2);
    }

    covered
}

fn find_range_gap(ranges: &Vec<(i64, i64)>) -> Option<i64> {
    let mut current_x_max = 0;
    for &(x1, x2) in ranges.iter() {
        if x1 > current_x_max {
            return Some(x1 - 1);
        }
        current_x_max = max(current_x_max, x2);
    }

    None
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let sensors = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<Sensor>>();

    {
        let ranges = covered_ranges(2000000, &sensors);
        println!("Covered on y=2000000: {}", merge_ranges(&ranges));
    }

    let limit = 4000000;
    let freq = (0..=limit)
        .map(|y| (y, covered_ranges(y, &sensors)))
        .map(|(y, ranges)| (y, find_range_gap(&ranges)))
        .find(|(_, gap)| gap.is_some())
        .map(|(y, x)| x.unwrap() * limit + y)
        .unwrap();
    println!("Frequency: {}", freq);
}
