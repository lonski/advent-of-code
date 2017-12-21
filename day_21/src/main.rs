extern crate itertools;

use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let rules = input.lines().map(Rule::new).collect();
    let image = generate_art(&rules, 5);
    println!("{}", &image.to_string());
    println!("Count of '#': {}", image.count('#'));
}

fn generate_art(rules: &Vec<Rule>, iterations: usize) -> Image {
    let mut image = Image::new();
    for i in 0..iterations {
        println!("Iteration {}", i);
        let size = image.size();
        let divisor = if size % 2 == 0 { 2 } else { 3 };
        let new_size = (size / divisor) * (divisor + 1);
        let mut new_image = Image::with_size(new_size);
        for (i, y) in (0..size).step(divisor).enumerate() {
            for (j, x) in (0..size).step(divisor).enumerate() {
                let pattern_to_find = image.sub_image(x, y, divisor);
                let rule = find_rule(&pattern_to_find, &rules);
                new_image.draw(&rule.replacement, x + j, y + i);
            }
        }
        image = new_image;
    }
    image
}

fn find_rule<'a>(pattern: &Image, rules: &'a Vec<Rule>) -> &'a Rule {
    rules.iter().find(|r| r.patterns.contains(pattern)).unwrap()
}

struct Rule {
    patterns: Vec<Image>,
    replacement: Image,
}

impl Rule {
    fn new(s: &str) -> Self {
        let mut rule_str = s.split(" => ");
        let pattern = Image::from(rule_str.next().unwrap());
        let replacement = Image::from(rule_str.next().unwrap());
        let patterns = vec![
            pattern.flip_h(),
            pattern.flip_v(),
            pattern.rotate(),
            pattern.rotate().flip_h(),
            pattern.rotate().flip_v(),
            pattern.rotate().rotate(),
            pattern.rotate().rotate().rotate(),
            pattern,
        ];
        Rule {
            patterns: patterns,
            replacement: replacement,
        }
    }
}

#[derive(PartialEq)]
struct Image {
    pixels: Vec<char>,
}

impl Image {
    fn with_size(size: usize) -> Image {
        Image { pixels: (0..size * size).map(|_| 'O').collect() }
    }
    fn from(pattern: &str) -> Self {
        Image { pixels: pattern.replace("/", "").chars().collect() }
    }
    fn new() -> Self {
        Image { pixels: ".#...####".chars().collect() }
    }
    fn size(&self) -> usize {
        (self.pixels.len() as f64).sqrt() as usize
    }
    fn sub_image(&self, x: usize, y: usize, size: usize) -> Image {
        let mut pixels: Vec<char> = Vec::new();
        for dy in 0..size {
            for dx in 0..size {
                pixels.push(self.pixel_at(x + dx, y + dy));
            }
        }
        Image { pixels: pixels }
    }
    fn draw(&mut self, image: &Image, draw_x: usize, draw_y: usize) {
        let size = image.size();
        for y in 0..size {
            for x in 0..size {
                let pix = image.pixel_at(x, y);
                self.set(draw_x + x, draw_y + y, pix);
            }
        }
    }
    fn pixel_at(&self, x: usize, y: usize) -> char {
        let pos = y * self.size() + x;
        if pos < self.pixels.len() {
            self.pixels[pos]
        } else {
            println!("Pixel out of bounds: ({},{})", x, y);
            ' '
        }
    }
    fn set(&mut self, x: usize, y: usize, pixel: char) -> bool {
        let pos = y * self.size() + x;
        if pos < self.pixels.len() {
            self.pixels[pos] = pixel;
            true
        } else {
            println!("Pixel out of bounds: ({},{})", x, y);
            false
        }
    }
    fn rotate(&self) -> Image {
        let size = self.size();
        let mut rotated = Image::with_size(size);
        for x in 0..size {
            for y in 0..size {
                rotated.set(size - 1 - y, x, self.pixel_at(x, y));
            }
        }
        rotated
    }
    fn flip_h(&self) -> Image {
        let size = self.size();
        let mut flipped = Image::with_size(size);
        for x in 0..size {
            for y in 0..size {
                flipped.set(x, size - 1 - y, self.pixel_at(x, y));
            }
        }
        flipped
    }
    fn flip_v(&self) -> Image {
        let size = self.size();
        let mut flipped = Image::with_size(size);
        for x in 0..size {
            for y in 0..size {
                flipped.set(size - 1 - x, y, self.pixel_at(x, y));
            }
        }
        flipped
    }
    fn count(&self, pix: char) -> usize {
        self.pixels.iter().filter(|p| **p == pix).count()
    }
    fn to_string(&self) -> String {
        let size = self.size();
        self.pixels
            .chunks(size)
            .map(|c| c.iter().collect::<String>())
            .fold(String::new(), |acc, row| format!("{}\n{}", &acc, &row))
    }
}
