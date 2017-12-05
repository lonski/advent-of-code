use std::collections::HashMap;

fn main() {
    println!("Part one: {}", part_one(312051));
    println!("Part two: {}", part_two(312051));
}

//--- part two stuff below

fn part_two(input: u32) -> u32 {
    let mut grid = HashMap::new();
    let mut level = 0;
    let mut size = 1;
    grid.insert((0, 0), 1);

    loop {
        level += 1;
        size += 2;
        let (mut x, mut y) = (level, -1 * (level - 1));
        let square_walks = vec![
            (0..1, (0, 0)),
            (1..size - 1, (0, 1)),
            (1..size, (-1, 0)),
            (1..size, (0, -1)),
            (1..size, (1, 0)),
        ];
        for (range, (x_mod, y_mod)) in square_walks {
            for _ in range.clone() {
                x += x_mod;
                y += y_mod;
                let value = fill(x, y, &mut grid);
                if value > input {
                    return value;
                }
            }
        }
    }

}

fn fill(x: i32, y: i32, grid: &mut HashMap<(i32, i32), u32>) -> u32 {
    let val = get_val(x, y, &grid);
    grid.insert((x, y), val);
    *grid.get(&(x, y)).unwrap()
}

fn get_val(x: i32, y: i32, grid: &HashMap<(i32, i32), u32>) -> u32 {
    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, 0),
    ].iter()
        .filter_map(|&(x_mod, y_mod)| grid.get(&(x + x_mod, y + y_mod)))
        .fold(0, |acc, &v| acc + v)

}

//--- part one stuff below

fn part_one(input: u32) -> u32 {
    let square = Square::of(input);
    let (x, y) = square.find_coords(input);
    (x.abs() + y.abs()) as u32
}

struct Square {
    x_start: i32,
    y_start: i32,
    size: u32,
}

impl Square {
    fn of(input: u32) -> Square {
        let size = Square::calc_square_size(input);
        let (x_start, y_start) = Square::calc_start_coords(size);
        Square {
            size: size,
            x_start: x_start,
            y_start: y_start,
        }
    }

    fn calc_square_size(n: u32) -> u32 {
        let x = (n as f32).sqrt().ceil() as u32;
        if x % 2 == 0 { x + 1 } else { x }
    }

    fn calc_start_coords(square_size: u32) -> (i32, i32) {
        let prev_square_size: i32 = square_size as i32 - 2;
        (
            prev_square_size / 2 as i32 + 1,
            -1 * prev_square_size / 2 as i32,
        )
    }

    fn find_coords(&self, to_find: u32) -> (i32, i32) {
        let mut x = self.x_start;
        let mut y = self.y_start;
        let mut num = (self.size - 2).pow(2) + 1;
        let square_walks = vec![
            (1..self.size - 1, (0, 1)),
            (1..self.size, (-1, 0)),
            (1..self.size, (0, -1)),
            (1..self.size, (1, 0)),
        ];
        for (range, (x_mod, y_mod)) in square_walks {
            for _ in range.clone() {
                x += x_mod;
                y += y_mod;
                num += 1;
                if num == to_find {
                    return (x, y);
                }
            }
        }
        (self.x_start, self.y_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_correct_square_size() {
        for n in 2..10 {
            assert_eq!(Square::calc_square_size(n), 3);
        }
        for n in 10..26 {
            assert_eq!(Square::calc_square_size(n), 5);
        }
        for n in 26..50 {
            assert_eq!(Square::calc_square_size(n), 7);
        }
    }

    #[test]
    fn should_calc_correct_start_coords() {
        assert_eq!(Square::calc_start_coords(3), (1, 0));
        assert_eq!(Square::calc_start_coords(5), (2, -1));
        assert_eq!(Square::calc_start_coords(7), (3, -2));
    }

    #[test]
    fn should_find_coords() {
        assert_eq!(part_one(12), 3);
        assert_eq!(part_one(23), 2);
        assert_eq!(part_one(1024), 31);
        assert_eq!(part_one(312051), 430);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(312051), 312453);
    }

}
