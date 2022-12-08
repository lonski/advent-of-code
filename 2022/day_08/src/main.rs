use std::cmp::max;
use std::collections::HashSet;
use std::{env, fs};

fn parse_map(input: String) -> (usize, usize, Vec<i32>) {
    let lines = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();
    let map = input
        .chars()
        .map(|c| c.to_string().parse::<i32>())
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap())
        .collect::<Vec<i32>>();

    (width, height, map)
}

fn find_visible_trees(width: usize, height: usize, map: &Vec<i32>) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // rows from both sides
    for y in 0..height {
        let mut max_from_left: i32 = -1;
        let mut max_from_right: i32 = -1;
        for x in 0..width {
            let from_left_h = map[y * width + x];
            if from_left_h > max_from_left {
                visible.insert((x, y));
            }
            max_from_left = max(max_from_left, from_left_h);

            let rx = width - x - 1;
            let from_right_h = map[y * width + rx];
            if from_right_h > max_from_right {
                visible.insert((rx, y));
            }
            max_from_right = max(max_from_right, from_right_h);
        }
    }

    // cols from both sides
    for x in 0..width {
        let mut max_from_top = -1;
        let mut max_from_bottom = -1;
        for y in 0..height {
            let from_top_h = map[y * width + x];
            if from_top_h > max_from_top {
                visible.insert((x, y));
            }
            max_from_top = max(max_from_top, from_top_h);

            let ry = height - y - 1;
            let from_bottom_h = map[ry * width + x];
            if from_bottom_h > max_from_bottom {
                visible.insert((x, ry));
            }
            max_from_bottom = max(max_from_bottom, from_bottom_h);
        }
    }

    visible
}

fn find_best_spot(
    width: usize,
    height: usize,
    map: &Vec<i32>,
    visible_trees: &HashSet<(usize, usize)>,
) -> i32 {
    visible_trees
        .iter()
        .map(|&(vx, vy)| {
            let curr = map[width * vy + vx];
            let mut score;

            //left
            let mut sum = 0;
            for x in (0..vx).rev() {
                sum += 1;
                if map[width * vy + x] >= curr {
                    break;
                }
            }
            score = sum;

            //right
            sum = 0;
            for x in vx + 1..width {
                sum += 1;
                if map[width * vy + x] >= curr {
                    break;
                }
            }
            score = score * sum;

            //top
            sum = 0;
            for y in (0..vy).rev() {
                sum += 1;
                if map[width * y + vx] >= curr {
                    break;
                }
            }
            score = score * sum;

            //bottom
            sum = 0;
            for y in vy + 1..height {
                sum += 1;
                if map[width * y + vx] >= curr {
                    break;
                }
            }
            score = score * sum;

            score
        })
        .max()
        .unwrap()
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let (width, height, map) = parse_map(input);

    let visible_trees = find_visible_trees(width, height, &map);
    println!("Visible: {}", visible_trees.len());
    println!(
        "Max score: {}",
        find_best_spot(width, height, &map, &visible_trees)
    );
}
