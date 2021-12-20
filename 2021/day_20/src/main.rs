use std::env;
use std::fs;
use std::io::{self, Write};

fn expand(image: &mut Vec<Vec<bool>>, val: bool) {
    let new_row = std::iter::repeat(val)
        .take(image[0].len())
        .collect::<Vec<bool>>();
    image.insert(0, new_row.clone());
    image.push(new_row.clone());
    for v in image {
        v.insert(0, val);
        v.push(val);
    }
}

fn parse_input(input: String) -> (String, Vec<Vec<bool>>) {
    let mut split = input.split("\n");

    let alg = split.nth(0).unwrap();
    let mut image = split
        .skip(1)
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    expand(&mut image, false);

    (alg.to_string(), image)
}

fn get_image_val(p: &(i32, i32), image: &Vec<Vec<bool>>, def_val: bool) -> bool {
    let w = image[0].len() as i32;
    let h = image.len() as i32;

    if p.0 < 0 || p.0 >= w || p.1 < 0 || p.1 >= h {
        return def_val;
    }

    image[p.1 as usize][p.0 as usize]
}

fn calculate_index(p: (i32, i32), image: &Vec<Vec<bool>>, def_val: bool) -> usize {
    let binary = [
        (p.0 - 1, p.1 - 1),
        (p.0, p.1 - 1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1),
        (p.0, p.1),
        (p.0 + 1, p.1),
        (p.0 - 1, p.1 + 1),
        (p.0, p.1 + 1),
        (p.0 + 1, p.1 + 1),
    ]
    .iter()
    .map(|p| get_image_val(p, image, def_val))
    .map(|v| if v { '1' } else { '0' })
    .collect::<String>();

    usize::from_str_radix(&binary, 2).unwrap()
}

fn enchance(image: &Vec<Vec<bool>>, alg: &String, def_val: bool) -> Vec<Vec<bool>> {
    let mut enchanced = Vec::new();

    for y in 0..image.len() {
        let mut row: Vec<bool> = Vec::new();
        for x in 0..image[0].len() {
            let index = calculate_index((x as i32, y as i32), &image, def_val);
            row.push(alg.chars().nth(index).unwrap() == '#');
        }
        enchanced.push(row);
    }
    enchanced
}

fn count_lit(image: &Vec<Vec<bool>>) -> usize {
    image
        .iter()
        .map(|v| v.iter().filter(|v| **v == true).count())
        .sum::<usize>()
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let input = fs::read_to_string(filename).unwrap();
    let (alg, mut image) = parse_input(input);

    let mut def_val = false;
    for i in 0..50 {
        print!(".");

        image = enchance(&image, &alg, def_val);
        expand(&mut image, !def_val);

        def_val = !def_val;
        if i == 1 {
            println!("Part 1 = {}", count_lit(&image));
        }

        io::stdout().flush().unwrap();
    }

    println!("Part 2 = {}", count_lit(&image));
}
