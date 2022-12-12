use std::{env, fs};

use pathfinding::prelude::bfs;

struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: String) -> Self {
        let lines = input
            .split("\n")
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();
        Map {
            tiles: input.chars().filter(|&c| c != '\n').collect::<Vec<char>>(),
            width: lines[0].len(),
            height: lines.len(),
        }
    }

    fn coord(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    fn possible_exists(&self, pos: usize) -> Vec<usize> {
        let (x, y) = self.coord(pos);
        vec![
            self.idx(x + 1, y),
            self.idx(x - 1, y),
            self.idx(x, y + 1),
            self.idx(x, y - 1),
        ]
        .into_iter()
        .filter(|&idx| idx < self.width * self.height)
        .filter(|&nb| self.elevation(nb as usize) - self.elevation(pos) <= 1)
        .collect::<Vec<usize>>()
    }

    fn elevation(&self, idx: usize) -> i32 {
        match self.tiles.get(idx) {
            Some('S') => 'a' as i32,
            Some('E') => 'z' as i32,
            Some(x) => *x as i32,
            _ => panic!(),
        }
    }

    fn path_length(&self, start: usize, end: usize) -> Option<usize> {
        bfs(&start, |pos| self.possible_exists(*pos), |pos| *pos == end).map(|r| r.len() - 1)
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let map = Map::parse(input);
    let mut start = 0;
    let mut end = 0;
    let mut lowest: Vec<usize> = Vec::new();
    map.tiles.iter().enumerate().for_each(|(idx, c)| match c {
        'S' => start = idx,
        'E' => end = idx,
        'a' => lowest.push(idx),
        _ => {}
    });

    println!("Part 1: {}", map.path_length(start, end).unwrap());
    println!(
        "Part 2: {}",
        lowest
            .iter()
            .map(|&i| map.path_length(i, end))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .min()
            .unwrap()
    );
}
