use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();
    input.pop();
    println!("(Part one, Part two): {:?}", solve(&input));
}

fn solve(input: &str) -> (i32, i32) {
    let start = Hex::new();
    let mut goal = Hex::new();
    let mut max: i32 = 0;
    for dir in input.split(",") {
        goal.move_to(dir);
        max = cmp::max(max, start.distance(&goal));
    }
    (start.distance(&goal), max)
}

#[derive(Debug)]
struct Hex {
    x: i32,
    y: i32,
    z: i32,
}

impl Hex {
    fn new() -> Hex {
        Hex { x: 0, y: 0, z: 0 }
    }

    fn move_to(&mut self, dir: &str) {
        match dir {
            "n" => {
                self.y += 1;
                self.z -= 1;
            }
            "ne" => {
                self.x += 1;
                self.z -= 1;
            }
            "se" => {
                self.y -= 1;
                self.x += 1;
            }
            "s" => {
                self.y -= 1;
                self.z += 1;
            }
            "sw" => {
                self.z += 1;
                self.x -= 1;
            }
            "nw" => {
                self.y += 1;
                self.x -= 1;
            }
            _ => panic!(format!("Unsupported direction: {:?}", dir)),
        }
    }

    fn distance(&self, other: &Hex) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}
