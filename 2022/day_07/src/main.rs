use std::collections::HashMap;
use std::collections::HashSet;
use std::{env, fs};

struct Dir {
    id: u32,
    name: String,
    parent: u32,
    sub_dirs: HashSet<u32>,
    files: HashSet<(u32, String)>,
}

impl Dir {
    fn new(id: u32, parent: u32, name: &str) -> Dir {
        Dir {
            id,
            parent,
            name: String::from(name),
            sub_dirs: HashSet::new(),
            files: HashSet::new(),
        }
    }
}

struct IdGen {
    current: u32,
}

impl IdGen {
    fn new() -> IdGen {
        IdGen { current: 0 }
    }

    fn next(&mut self) -> u32 {
        self.current += 1;
        self.current
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let mut id_gen = IdGen::new();
    let mut fs: HashMap<u32, Dir> = HashMap::new();
    fs.insert(id_gen.current, Dir::new(id_gen.current, 0, "/"));

    parse_file_system(input, &mut fs, &mut id_gen);

    let dir_sizes = fs
        .keys()
        .cloned()
        .collect::<Vec<u32>>()
        .iter()
        .map(|&dir_id| calc_dir_size(dir_id, &mut fs))
        .collect::<Vec<u32>>();

    println!(
        "Sum of dir sizes less than 100000: {:?}",
        dir_sizes
            .iter()
            .filter(|&&size| size <= 100000)
            .sum::<u32>()
    );

    let total_used = calc_dir_size(0, &mut fs);
    let free = 70000000 - total_used;
    let to_free = 30000000 - free;
    println!(
        "Size of dir to delete: {}",
        dir_sizes.iter().filter(|&&s| s >= to_free).min().unwrap()
    );
}

fn calc_dir_size(dir_id: u32, fs: &mut HashMap<u32, Dir>) -> u32 {
    let mut dirs = vec![dir_id];
    let mut size = 0;
    while !dirs.is_empty() {
        let current = dirs.pop().unwrap();
        let dir = fs.get(&current).unwrap();
        size += dir.files.iter().map(|(s, _)| s).sum::<u32>();
        dir.sub_dirs.iter().for_each(|&sub| dirs.push(sub));
    }
    size
}

fn parse_file_system(input: String, mut fs: &mut HashMap<u32, Dir>, mut id_gen: &mut IdGen) {
    let lines = input.split("\n").skip(1).filter(|l| !l.is_empty());
    let mut current = id_gen.current;
    for line in lines {
        if line == "$ ls" {
        } else if line == "$ cd .." {
            current = fs.get(&current).unwrap().parent;
        } else if line.starts_with("$ cd") {
            current = find_or_create_sub_dir(current, &line[5..], &mut fs, &mut id_gen);
        } else if line.starts_with("dir ") {
            find_or_create_sub_dir(current, &line[4..], &mut fs, &mut id_gen);
        } else {
            let mut split = line.split(" ");
            let size = split.next().unwrap().parse::<u32>().unwrap();
            let file_name = split.next().unwrap();
            fs.get_mut(&current)
                .unwrap()
                .files
                .insert((size, String::from(file_name)));
        }
    }
}

fn find_or_create_sub_dir(
    current_dir_id: u32,
    sub_dir_name: &str,
    fs: &mut HashMap<u32, Dir>,
    id_gen: &mut IdGen,
) -> u32 {
    let sub_dirs = fs.get(&current_dir_id).unwrap().sub_dirs.clone();
    match sub_dirs
        .iter()
        .find(|id| fs.get(id).unwrap().name == sub_dir_name)
    {
        Some(&sub_dir_id) => sub_dir_id,
        None => {
            let new_id = id_gen.next();
            let new_sub_dir = Dir::new(new_id, current_dir_id, sub_dir_name);
            fs.insert(new_sub_dir.id, new_sub_dir);
            fs.get_mut(&current_dir_id).unwrap().sub_dirs.insert(new_id);
            new_id
        }
    }
}
