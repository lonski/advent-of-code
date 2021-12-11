use std::collections::HashSet;
use std::env;
use std::fs;

fn neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut nb = Vec::new();

    if x > 0 {
        nb.push((x - 1, y));
    }
    if y > 0 {
        nb.push((x, y - 1));
    }
    if x < (width - 1) {
        nb.push((x + 1, y));
    }
    if y < (height - 1) {
        nb.push((x, y + 1));
    }
    if y < (height - 1) && x < (width - 1) {
        nb.push((x + 1, y + 1));
    }
    if y < (height - 1) && x > 0 {
        nb.push((x - 1, y + 1));
    }
    if y > 0 && x > 0 {
        nb.push((x - 1, y - 1));
    }
    if y > 0 && x < (width - 1) {
        nb.push((x + 1, y - 1));
    }

    nb
}

fn flash(x: usize, y: usize, mut map: &mut Vec<Vec<u32>>, mut flashed: &mut HashSet<(usize, usize)>) {
    let height = map.len();
    let width = map[0].len();
    if flashed.contains(&(x, y)) {
        return;
    }

    let val = map[y][x];
    if val < 9 {
        map[y][x] = val + 1
    } else {
        map[y][x] = 0;
        flashed.insert((x, y));
        for (nbx, nby) in neighbours(x, y, width, height).iter() {
            flash(*nbx, *nby, &mut map, &mut flashed);
        }
    }
}

fn simulate(mut map: &mut Vec<Vec<u32>>) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            flash(x, y, &mut map, &mut flashed);
        }
    }

    flashed.len()
}

fn part_one(mut map: Vec<Vec<u32>>) {
    let mut flashed = 0;
    for _ in 0..100 {
        flashed += simulate(&mut map);
    }

    println!("Part 1: {}", flashed);
}

fn part_two(mut map: Vec<Vec<u32>>) {
  let mut steps = 1;
  let octo_count = map[0].len() * map.len();
  while simulate(&mut map) != octo_count {
    steps += 1
  }
  
  println!("Part 2: {}", steps);

}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let map: Vec<Vec<u32>> = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    part_one(map.clone());
    part_two(map.clone());
}
