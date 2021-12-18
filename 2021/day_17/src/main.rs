use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;

#[derive(Debug)]
struct TargetArea {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl TargetArea {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        if x >= self.x_range.0 && x <= self.x_range.1 {
            if y >= self.y_range.0 && y <= self.y_range.1 {
                return true;
            }
        }
        false
    }
}

fn parse_target_area(input: String) -> TargetArea {
    let x_range = &input.split(", ").nth(0).unwrap()[2..]
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let y_range = &input.split(", ").nth(1).unwrap()[2..]
        .split("..")
        .map(|y| y.parse().unwrap())
        .collect::<Vec<i32>>();

    TargetArea {
        x_range: (x_range[0], x_range[1]),
        y_range: (y_range[0], y_range[1]),
    }
}

fn simulate(vx_start: i32, vy_start: i32, target_area: &TargetArea) -> Option<i32> {
    let mut vx = vx_start;
    let mut vy = vy_start;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_y: i32 = 0;

    loop {
        x += vx;
        y += vy;
        vx = match vx.cmp(&0) {
            Ordering::Less => vx + 1,
            Ordering::Greater => vx - 1,
            Ordering::Equal => 0,
        };
        vy -= 1;
        max_y = std::cmp::max(y, max_y);

        // fell out target range
        if x > target_area.x_range.1 || y < target_area.y_range.0 {
            break;
        }

        if target_area.is_inside(x, y) {
            return Some(max_y);
        }
    }

    None
}

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Please provide input in the form: \"x=248..285, y=-85..-56\"");
    let target_area = parse_target_area(input);
    println!("{:?}", target_area);

    let mut y_max_total = 0;
    let mut valid: HashSet<(i32, i32)> = HashSet::new();
    for v_x in -500..500 {
        for v_y in -500..500 {
            if let Some(y_max) = simulate(v_x, v_y, &target_area) {
                y_max_total = std::cmp::max(y_max_total, y_max);
                valid.insert((v_x, v_y));
            }
        }
    }

    println!("Part 1: {}", y_max_total);
    println!("Part 2: {}", valid.len());
}
