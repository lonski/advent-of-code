use std::collections::HashSet;
use std::{env, fs};

fn move_head(&(cx, cy): &(i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "R" => (cx + 1, cy),
        "L" => (cx - 1, cy),
        "U" => (cx, cy + 1),
        "D" => (cx, cy - 1),
        _ => panic!("Unknown direction {}", dir),
    }
}

fn move_tail(&(tx, ty): &(i32, i32), &(hx, hy): &(i32, i32)) -> (i32, i32) {
    let dx: i32 = hx - tx;
    let dy: i32 = hy - ty;
    match (dx, dy) {
        (0, 0) => (tx, ty),
        (1, 0) => (tx, ty),
        (1, 1) => (tx, ty),
        (0, 1) => (tx, ty),
        (-1, 0) => (tx, ty),
        (-1, -1) => (tx, ty),
        (0, -1) => (tx, ty),
        (-1, 1) => (tx, ty),
        (1, -1) => (tx, ty),
        (2, 2) => (tx + 1, ty + 1),
        (-2, 2) => (tx - 1, ty + 1),
        (-2, -2) => (tx - 1, ty - 1),
        (2, -2) => (tx + 1, ty - 1),
        (0, 2) => (tx, ty + 1),
        (0, -2) => (tx, ty - 1),
        (2, 0) => (tx + 1, ty),
        (-2, 0) => (tx - 1, ty),
        (1, 2) => (tx + 1, ty + 1),
        (-1, 2) => (tx - 1, ty + 1),
        (1, -2) => (tx + 1, ty - 1),
        (-1, -2) => (tx - 1, ty - 1),
        (2, 1) => (tx + 1, ty + 1),
        (2, -1) => (tx + 1, ty - 1),
        (-2, 1) => (tx - 1, ty + 1),
        (-2, -1) => (tx - 1, ty - 1),

        _ => panic!("Unhandled dx/dy: {:?}", (dx, dy)),
    }
}

fn simulate_rope(rope_size: usize, moves: &Vec<(&str, i32)>) -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); rope_size];
    tail_visited.insert(*rope.last().unwrap());

    for &(dir, times) in moves {
        for _ in 0..times {
            rope[0] = move_head(&rope[0], dir);
            for r in 1..rope.len() {
                let segment_pos = move_tail(&rope[r], &rope[r - 1]);
                rope[r] = segment_pos;
            }
            tail_visited.insert(*rope.last().unwrap());
        }
    }

    tail_visited.len()
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let moves = input
        .lines()
        .map(|l| l.split(" "))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();

    println!("Short rope simulation: {:?}", simulate_rope(2, &moves));
    println!("Long rope simulation: {:?}", simulate_rope(10, &moves));
}
