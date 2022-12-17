use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;

type Position = (i32, i32);
const BITS: [u8; 7] = [128, 64, 32, 16, 8, 4, 2];

#[derive(Copy, Clone, Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Clone)]
struct Rock {
    tiles: Vec<Position>,
    width: i32,
}

impl Rock {
    fn new(tiles: Vec<Position>, width: i32) -> Self {
        Rock { tiles, width }
    }
}

struct Cave {
    rock_tiles: Vec<u8>,
    width: i32,
    rocks: Vec<Rock>,
    jets: Vec<Jet>,
    jet_idx: i32,
    rock_idx: i32,
    rocks_settled: i64,
    heights: HashMap<i64, i64>,
}

impl Cave {
    fn next_jet(&mut self) -> Jet {
        self.jet_idx = (self.jet_idx + 1) % self.jets.len() as i32;
        self.jets[self.jet_idx as usize]
    }

    fn next_rock(&mut self) -> (Rock, Position) {
        self.rock_idx = (self.rock_idx + 1) % self.rocks.len() as i32;
        (
            self.rocks[self.rock_idx as usize].clone(),
            (2, self.tower_height() as i32 + 3), // starting pos
        )
    }

    fn new(rocks: Vec<Rock>, jets: Vec<Jet>) -> Self {
        Cave {
            rock_tiles: Vec::new(),
            width: 7,
            rocks,
            jets,
            jet_idx: -1,
            rock_idx: -1,
            rocks_settled: 0,
            heights: HashMap::new(),
        }
    }

    fn move_rock(&self, rock: &Rock, pos: Position, jet: Jet) -> Position {
        let new_pos = match jet {
            Jet::Left => (pos.0 - 1, pos.1),
            Jet::Right => (pos.0 + 1, pos.1),
        };

        if !self.can_put(&rock, new_pos) {
            return pos;
        }

        new_pos
    }

    fn fall_rock(&self, rock: &Rock, pos: Position) -> Option<Position> {
        let new_pos = (pos.0, pos.1 - 1);
        if !self.can_put(&rock, new_pos) {
            return None;
        }

        return Some(new_pos);
    }

    fn set(&mut self, pos: Position) {
        while self.rock_tiles.len() < (pos.1 + 1) as usize {
            self.rock_tiles.push(0);
        }
        // 0b1000000 -> 128 -> x=0
        // 0b0100000 -> 64  -> x=1
        // 0b0010000 -> 32  -> x=2
        // 0b0001000 -> 16  -> x=3
        // 0b0000100 -> 8   -> x=4
        // 0b0000010 -> 4   -> x=5
        // 0b0000001 -> 2   -> x=6

        // println!("set {:?}", pos);
        self.rock_tiles[pos.1 as usize] |= BITS[pos.0 as usize];
    }

    fn is_rock(&self, pos: Position) -> bool {
        if pos.0 < 0 || pos.0 > 6 || (pos.1 as usize) >= self.rock_tiles.len() {
            return false;
        }
        (self.rock_tiles[pos.1 as usize] & BITS[pos.0 as usize]) != 0
    }

    fn settle(&mut self, rock: &Rock, pos: Position) {
        rock.tiles
            .iter()
            .map(|(x, y)| (x + pos.0, y + pos.1))
            .for_each(|pos| {
                self.set(pos);
            });
        self.rocks_settled += 1;
        self.heights
            .insert(self.rocks_settled, self.tower_height() as i64);
    }

    fn can_put(&self, rock: &Rock, new_pos: (i32, i32)) -> bool {
        //hit floor
        if new_pos.1 < 0 {
            return false;
        }

        // boundaries check
        if new_pos.0 < 0 || new_pos.0 + rock.width > self.width {
            return false;
        }

        // collide check
        rock.tiles
            .iter()
            .map(|(x, y)| (x + new_pos.0, y + new_pos.1))
            .find(|pos| self.is_rock(*pos))
            .is_none()
    }

    fn tower_height(&self) -> usize {
        self.rock_tiles.len()
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (-1..(self.tower_height() as i32 + 1)).rev() {
            for x in -1..8 {
                if self.is_rock((x, y)) {
                    write!(f, "#").unwrap();
                } else {
                    if y == -1 {
                        write!(f, "-").unwrap();
                    } else {
                        match x {
                            -1 => write!(f, "|").unwrap(),
                            7 => write!(f, "|").unwrap(),
                            _ => write!(f, ".").unwrap(),
                        };
                    }
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

fn init_state(input: String) -> Cave {
    let gas_jets = input
        .chars()
        .filter(|&c| c == '>' || c == '<')
        .map(|c| if c == '>' { Jet::Right } else { Jet::Left })
        .collect::<Vec<Jet>>();
    let rock_shapes: Vec<Rock> = vec![
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)], 4),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 3),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)], 1),
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)], 2),
    ];

    Cave::new(rock_shapes, gas_jets)
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let mut cave = init_state(input);
    let mut cycles: HashMap<(i32, i32), (i64, i64)> = HashMap::new(); // (jet_index, rock_index) -> (tower_height, rocks_settled)

    loop {
        let (rock, mut pos) = cave.next_rock();
        let mut jet = cave.next_jet();
        pos = cave.move_rock(&rock, pos, jet);

        loop {
            match cave.fall_rock(&rock, pos) {
                Some(new_pos) => {
                    pos = new_pos;
                    jet = cave.next_jet();
                    pos = cave.move_rock(&rock, pos, jet);
                }
                None => {
                    cave.settle(&rock, pos);
                    break;
                }
            }
        }

        // Look for cycles
        // When fully filled floor happens,
        // save current gas jet and rock position along with settled rock count and tower height
        //
        // At second cycle calculate how many rocks fell during the cycle and what height was produced.
        // Divide the remaining rock count (target 1kkkk value minus height before first cycle) by
        // the rock count that fell in one cycle and multiply it by the height gained by single cycle
        //
        // It may produce results wrong by single rocks as float precision calculations are done
        // Could be probably improved by using the modulo reminder and get how many rocks fell in part of the cycle
        if cave.rock_tiles.last() == Some(&(0b11111110 as u8)) {
            let state = (cave.jet_idx, cave.rock_idx);
            if let Some(&(last_cycle_height, last_cycle_rocks_settled)) = cycles.get(&state) {
                let rocks_per_cycle = (cave.rocks_settled - last_cycle_rocks_settled) as f64;
                let height_per_cycle = cave.tower_height() as f64 - last_cycle_height as f64;
                let rocks_left = (1000000000000 - last_cycle_rocks_settled - 1) as f64;
                let height_at_1kkkk =
                    last_cycle_height as f64 + (rocks_left / rocks_per_cycle * height_per_cycle);
                cave.heights.insert(1000000000000, height_at_1kkkk as i64);
                break;
            } else {
                cycles.insert(
                    (cave.jet_idx, cave.rock_idx),
                    (cave.tower_height() as i64, cave.rocks_settled),
                );
            }
        }
    }

    println!(
        "Height after 2022 rocks: {}",
        cave.heights.get(&2022).unwrap()
    );
    println!(
        "Height after 1000000000000 rocks: {}",
        cave.heights.get(&1000000000000).unwrap()
    );
}
