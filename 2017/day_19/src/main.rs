use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("(Part one, Part two) = {:?}", walk(&input));
}

fn walk(input: &str) -> (String, usize) {
    let mut network = Network::new(input);
    let mut steps = 0;
    loop {
        steps += 1;
        if !network.next() {
            break;
        }
    }
    (network.letters.clone(), steps)
}

#[derive(Debug)]
struct Network {
    net: String,
    direction: Direction,
    pos: usize,
    width: usize,
    letters: String,
}

impl Network {
    fn new(input: &str) -> Self {
        let line_cnt = input.lines().count();
        let net: String = input.lines().collect();
        Network {
            direction: Direction::Down,
            pos: input.find('|').unwrap(),
            width: net.len() / line_cnt,
            net: net,
            letters: String::new(),
        }
    }

    fn next(&mut self) -> bool {
        self.pos = self.neighbour_pos(&self.direction);
        let current = self.current_char();
        match current {
            '+' => {
                self.change_direction();
            }
            'A'...'Z' => self.letters.push(current),
            _ => (),
        }
        self.current_char() != ' '
    }

    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up | Direction::Down => {
                if self.neighbour(&Direction::Left) != ' ' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left | Direction::Right => {
                if self.neighbour(&Direction::Up) != ' ' {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        };
    }

    fn current_char(&self) -> char {
        self.char_at(self.pos)
    }

    fn char_at(&self, pos: usize) -> char {
        self.net.chars().nth(pos).unwrap()
    }

    fn neighbour(&self, dir: &Direction) -> char {
        let nb_pos = self.neighbour_pos(dir);
        if nb_pos >= self.net.len() {
            return ' ';
        }
        self.char_at(nb_pos)
    }

    fn neighbour_pos(&self, dir: &Direction) -> usize {
        match *dir {
            Direction::Down => self.pos + self.width,
            Direction::Up => self.pos - self.width,
            Direction::Right => self.pos + 1,
            Direction::Left => self.pos - 1,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input: String = String::from(
            "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ ",
        );
        assert_eq!(walk(&input), (String::from("ABCDEF"), 38));
    }
}
