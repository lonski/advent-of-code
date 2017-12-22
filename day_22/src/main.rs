use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

}
fn part_two(input: &str) -> i64 {
    let mut grid = InfiniteGrid::new(&input);
    let mut virus = Virus::new();
    let mut infections = 0;
    for _ in 0..10000000 {
        match grid.get(virus.x, virus.y) {
            '.' => {
                virus.turn(Dir::L);
                grid.set(virus.x, virus.y, 'W');
            }
            'W' => {
                grid.set(virus.x, virus.y, '#');
                infections += 1;
            }
            '#' => {
                virus.turn(Dir::R);
                grid.set(virus.x, virus.y, 'F');
            }
            'F' => {
                virus.reverse_direction();
                grid.set(virus.x, virus.y, '.');
            }
            _ => panic!("Unknown state"),
        }
        virus.move_forward();
    }
    infections
}

fn part_one(input: &str) -> i64 {
    let mut grid = InfiniteGrid::new(&input);
    let mut virus = Virus::new();
    (0..10000)
        .map(|_| {
            let is_infected = grid.is_infected(virus.x, virus.y);
            virus.turn(if is_infected { Dir::R } else { Dir::L });
            grid.set(virus.x, virus.y, if is_infected { '.' } else { '#' });
            virus.move_forward();
            if is_infected { 0 } else { 1 }
        })
        .sum()
}

struct Virus {
    x: i64,
    y: i64,
    face: Dir,
}

impl Virus {
    fn new() -> Self {
        Virus {
            x: 0,
            y: 0,
            face: Dir::U,
        }
    }
    fn turn(&mut self, dir: Dir) {
        self.face = match self.face {
            Dir::U => if dir == Dir::L { Dir::L } else { Dir::R },
            Dir::L => if dir == Dir::L { Dir::D } else { Dir::U },
            Dir::D => if dir == Dir::L { Dir::R } else { Dir::L },
            Dir::R => if dir == Dir::L { Dir::U } else { Dir::D },
        };
    }
    fn move_forward(&mut self) {
        match &self.face {
            &Dir::U => self.y += 1,
            &Dir::L => self.x -= 1,
            &Dir::D => self.y -= 1,
            &Dir::R => self.x += 1,
        }
    }
    fn reverse_direction(&mut self) {
        self.turn(Dir::L);
        self.turn(Dir::L);
    }
}

#[derive(Debug)]
struct InfiniteGrid {
    cells: HashMap<(i64, i64), char>,
}

impl InfiniteGrid {
    fn new(init_state: &str) -> Self {
        let mut cells = HashMap::new();
        let y_max = init_state.lines().count() as i64;
        let x_max = init_state.lines().next().unwrap().len() as i64;
        for (y, row) in init_state.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let px = -1 * (x_max / 2) + x as i64;
                let py = (y_max / 2) - y as i64;
                cells.insert((px, py), c);
            }
        }
        InfiniteGrid { cells: cells }
    }
    fn is_infected(&self, x: i64, y: i64) -> bool {
        *self.cells.get(&(x, y)).unwrap_or(&'.') == '#'
    }
    fn set(&mut self, x: i64, y: i64, c: char) {
        self.cells.insert((x, y), c);
    }
    fn get(&self, x: i64, y: i64) -> char {
        *self.cells.get(&(x, y)).unwrap_or(&'.')
    }
}

#[derive(PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}
