use std::cmp::max;
use std::{env, fs};

mod visualisation;

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileType {
    Air,
    Wall,
    Sand,
}

pub struct Map {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
    snapshots: Vec<Vec<TileType>>,
    do_snaps: bool,
    snap_freq: usize,
    snap_counter: usize,
    max_y_wall: usize,
    x_shift: usize,
}

impl Map {
    fn new(input: String, do_snaps: bool, snap_freq: usize, floor: bool) -> Self {
        let width = 1300;
        let height = 180;

        let mut map = Map {
            tiles: vec![TileType::Air; width * height],
            width,
            height,
            max_y_wall: 0,
            x_shift: 500,
            snapshots: Vec::new(),
            do_snaps,
            snap_freq,
            snap_counter: 0,
        };

        let walls = input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(" -> ")
                    .map(|step| {
                        let mut nums = step.split(",");
                        (
                            nums.next().unwrap().parse::<i32>().unwrap(),
                            nums.next().unwrap().parse::<i32>().unwrap(),
                        )
                    })
                    .collect::<Vec<(i32, i32)>>()
            })
            .collect::<Vec<Vec<(i32, i32)>>>();

        for w in walls.iter() {
            let &(wx, wy) = w.iter().next().unwrap();
            let mut x = wx;
            let mut y = wy;
            map.set(x, y, TileType::Wall);
            for &(sx, sy) in w.iter().skip(1) {
                for dx in 0..=(sx - x).abs() {
                    match sx >= x {
                        true => map.set(x + dx, y, TileType::Wall),
                        false => map.set(x - dx, y, TileType::Wall),
                    }
                }
                for dy in 0..=(sy - y).abs() {
                    let ny;
                    match sy >= y {
                        true => ny = y + dy,
                        false => ny = y - dy,
                    }
                    map.set(x, ny, TileType::Wall);
                    map.max_y_wall = max(map.max_y_wall, ny as usize);
                }
                x = sx;
                y = sy;
            }
        }

        if floor {
            let fy = map.max_y_wall + 2;
            for fx in 0..map.width {
                map.set(fx as i32, fy as i32, TileType::Wall);
            }
        }

        let snap = map.tiles.clone();
        map.snapshots.push(snap);

        map
    }

    fn get(&self, x: i32, y: i32) -> Option<TileType> {
        let sx = self.x_shift as i32 + x;
        if sx < 0 || y < 0 || sx >= self.width as i32 || y >= (self.height - 1) as i32 {
            return None;
        }
        let idx = self.idx(sx, y);
        Some(self.tiles[idx])
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + (x + self.x_shift as i32)) as usize
    }

    fn set(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.idx(x + self.x_shift as i32, y);
        self.tiles[idx] = tile;
    }

    fn take_snap(&mut self) {
        self.snap_counter += 1;
        if self.do_snaps && self.snap_counter >= self.snap_freq {
            let snap = self.tiles.clone();
            self.snap_counter = 0;
            self.snapshots.push(snap);
        }
    }
}

fn pour_sand(map: &mut Map) -> usize {
    let mut sx: i32 = 500;
    let mut sy: i32 = 0;
    let mut sand_units: usize = 0;

    loop {
        if sy > (map.max_y_wall + 1) as i32 {
            break;
        }

        if let Some(bottom_tile) = map.get(sx, sy + 1) {
            if bottom_tile == TileType::Air {
                map.set(sx, sy, TileType::Air);
                sy += 1;
                map.set(sx, sy, TileType::Sand);
                map.take_snap();
            } else if map.get(sx - 1, sy + 1) == Some(TileType::Air) {
                map.set(sx, sy, TileType::Air);
                sx -= 1;
                sy += 1;
                map.set(sx, sy, TileType::Sand);
                map.take_snap();
            } else if map.get(sx + 1, sy + 1) == Some(TileType::Air) {
                map.set(sx, sy, TileType::Air);
                sx += 1;
                sy += 1;
                map.set(sx, sy, TileType::Sand);
                map.take_snap();
            } else if sx == 500 && sy == 0 {
                sand_units += 1;
                map.set(sx, sy, TileType::Sand);
                break;
            } else {
                sx = 500;
                sy = 0;
                sand_units += 1;
            }
        } else {
            break;
        }
    }

    sand_units
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let mut first_map = Map::new(input.clone(), true, 50, false);
    println!("{}", pour_sand(&mut first_map));

    let mut second_map = Map::new(input.clone(), true, 800, true);
    println!("{}", pour_sand(&mut second_map));

    visualisation::run(&first_map);
    // visualisation::run(&second_map);
}
