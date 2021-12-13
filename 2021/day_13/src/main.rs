use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_paper(input: String) -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let mut paper = HashSet::new();
    let mut folds = Vec::new();

    for line in input.split("\n").filter(|l| !l.is_empty()) {
        if line.starts_with("fold") {
            let fold = line.split(" ").nth(2).unwrap().split("=");
            folds.push((
                fold.clone().nth(0).unwrap().chars().next().unwrap(),
                fold.clone().nth(1).unwrap().parse().unwrap(),
            ));
        } else {
            let point = line.split(",");
            paper.insert((
                point.clone().nth(0).unwrap().parse().unwrap(),
                point.clone().nth(1).unwrap().parse().unwrap(),
            ));
        }
    }

    (paper, folds)
}

fn fold_point(x: u32, y: u32, fold_axis: char, fold: u32) -> (u32, u32) {
    if fold_axis == 'x' {
        if x < fold {
            (x, y)
        } else {
            (fold - (x - fold), y)
        }
    } else {
        if y < fold {
            (x, y)
        } else {
            (x, fold - (y - fold))
        }
    }
}

fn fold_paper(
    paper: &HashSet<(u32, u32)>,
    (fold_axis, fold_val): (char, u32),
) -> HashSet<(u32, u32)> {
    let mut folded = HashSet::new();

    for (x, y) in paper {
        folded.insert(fold_point(*x, *y, fold_axis, fold_val));
    }

    folded
}

fn print_paper(paper: &HashSet<(u32, u32)>) {
    for y in 0..6 {
        for x in 0..50 {
            if paper.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let (paper, folds) = parse_paper(fs::read_to_string(filename).unwrap());

    let folded = fold_paper(&paper, folds[0]);
    println!("Part 1: {}", folded.len());

    let folded = folds
        .iter()
        .fold(paper, |paper, fold| fold_paper(&paper, *fold));

    println!("Part 2:");
    print_paper(&folded);
}
