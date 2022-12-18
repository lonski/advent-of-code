use std::collections::HashSet;
use std::env;
use std::fs;

type Cube = (i32, i32, i32);

fn adjacent(c: &Cube) -> Vec<Cube> {
    vec![
        (c.0 - 1, c.1, c.2),
        (c.0 + 1, c.1, c.2),
        (c.0, c.1 - 1, c.2),
        (c.0, c.1 + 1, c.2),
        (c.0, c.1, c.2 + 1),
        (c.0, c.1, c.2 - 1),
    ]
}

fn surface(lava: &HashSet<Cube>) -> i32 {
    lava.iter()
        .map(|c| adjacent(c).iter().filter(|adj| !lava.contains(adj)).count() as i32)
        .sum::<i32>()
}

fn exterior_surface(lava: &HashSet<Cube>) -> i32 {
    let min = (
        lava.iter().map(|c| c.0).min().unwrap() - 1,
        lava.iter().map(|c| c.1).min().unwrap() - 1,
        lava.iter().map(|c| c.2).min().unwrap() - 1,
    );

    let max = (
        lava.iter().map(|c| c.0).max().unwrap() + 1,
        lava.iter().map(|c| c.1).max().unwrap() + 1,
        lava.iter().map(|c| c.2).max().unwrap() + 1,
    );

    // BFS to flood around the shape and find only outer sides
    let mut exposed = 0;
    let mut frontier = vec![min];
    let mut visited: HashSet<Cube> = HashSet::new();
    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        adjacent(&current)
            .into_iter()
            .filter(|&c| c.0 >= min.0 && c.1 >= min.1 && c.2 >= min.2)
            .filter(|&c| c.0 <= max.0 && c.1 <= max.1 && c.2 <= max.2)
            .for_each(|adj| {
                if lava.contains(&adj) {
                    exposed += 1;
                } else if !visited.contains(&adj) {
                    frontier.push(adj);
                }
                visited.insert(adj);
            });
    }

    exposed
}

fn parse_cubes(input: String) -> HashSet<Cube> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(","))
        .map(|mut s| {
            (
                s.next().unwrap().parse::<i32>().unwrap(),
                s.next().unwrap().parse::<i32>().unwrap(),
                s.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<HashSet<Cube>>()
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let lava = parse_cubes(input);

    println!("Surface: {}", surface(&lava));
    println!("Exterior surface: {}", exterior_surface(&lava));
}
