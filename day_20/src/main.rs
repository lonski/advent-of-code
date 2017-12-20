extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut simulation_p1 = Simulation::new(&input);
    let mut simulation_p2 = simulation_p1.clone();

    simulation_p1.run(500, false);
    println!("Part one: {}", simulation_p1.find_closest());

    simulation_p2.run(500, true);
    println!("Part two: {}", simulation_p2.particles.len());
}

#[derive(Debug, Clone)]
struct Simulation {
    particles: Vec<Particle>,
}

impl Simulation {
    fn new(input: &str) -> Self {
        Simulation {
            particles: input
                .lines()
                .enumerate()
                .map(|(i, s)| Particle::new(s, i))
                .collect(),
        }
    }
    fn run(&mut self, times: usize, handle_collisions: bool) {
        (0..times).for_each(|_| {
            self.particles.iter_mut().for_each(|p| {
                p.velocity.add(&p.acceleration);
                p.position.add(&p.velocity);
            });
            if handle_collisions {
                self.handle_collisions();
            }
        });
    }
    fn handle_collisions(&mut self) {
        let collided: Vec<usize> = self.particles
            .iter()
            .map(|p| {
                self.particles
                    .iter()
                    .filter(|o| o.position == p.position)
                    .filter(|o| o.id != p.id)
                    .map(|o| o.id)
                    .collect::<Vec<usize>>()
            })
            .fold(Vec::new(), |mut acc, v| {
                acc.extend(v);
                acc
            });
        self.particles.retain(|p| !collided.contains(&p.id));
    }
    fn find_closest(&self) -> usize {
        self.particles
            .iter()
            .map(|p| (p.id, p.distance()))
            .min_by_key(|&(_, dist)| dist)
            .map(|(id, _)| id)
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Particle {
    id: usize,
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn new(s: &str, id: usize) -> Self {
        let r = Regex::new(
            r"p=<\s?(-?\d+),\s?(-?\d+),\s?(-?\d+)>, v=<\s?(-?\d+),\s?(-?\d+),\s?(-?\d+)>, a=<\s?(-?\d+),\s?(-?\d+),\s?(-?\d+)>").unwrap();
        let v: Vec<i64> = r.captures(s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str())
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        Particle {
            id: id,
            position: Vec3::new(v[0], v[1], v[2]),
            velocity: Vec3::new(v[3], v[4], v[5]),
            acceleration: Vec3::new(v[6], v[7], v[8]),
        }
    }
    fn distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x, y, z }
    }
    fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_particle() {
        let p = Particle::new("p=<-3897,-624,3460>, v=<120,36,11>, a=<-1,-1,-5>");
        assert_eq!(p.position, Vec3::new(-3897, -624, 3460));
        assert_eq!(p.velocity, Vec3::new(120, 36, 11));
        assert_eq!(p.acceleration, Vec3::new(-1, -1, -5));
    }
}
