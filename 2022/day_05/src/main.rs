use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let initial_state = parse_state(parts[0]);
    let moves = parse_moves(parts[1]);

    println!("{:?}", crane_9000(initial_state.clone(), &moves));
    println!("{:?}", crane_9001(initial_state.clone(), &moves));
}

fn crane_9000(mut state: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    moves.iter().for_each(|(count, from, to)| {
        for _ in 0..*count {
            let to_move = state[*from].pop().unwrap();
            state[*to].push(to_move);
        }
    });
    state.iter().map(|s| s[s.len() - 1]).collect::<String>()
}

fn crane_9001(mut state: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    moves.iter().for_each(|(count, from, to)| {
        for t in 0..*count {
            let to_move = state[*from].pop().unwrap();
            let to_size = state[*to].len();
            state[*to].insert(to_size - t, to_move);
        }
    });
    state.iter().map(|s| s[s.len() - 1]).collect::<String>()
}

fn parse_state(state_string: &str) -> Vec<Vec<char>> {
    let mut initial_state = state_string
        .split("\n")
        .filter(|line| !line.is_empty() && !line.starts_with(" 1"))
        .map(|line| line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chunk| chunk.iter().find(|c| c.is_alphabetic()).unwrap_or(&' '))
            .map(|c| *c)
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();
    initial_state.reverse();

    let mut state: Vec<Vec<char>> = vec![Vec::new(); initial_state[0].len()];
    initial_state.iter().for_each(|row| row
        .iter()
        .enumerate()
        .filter(|(_, val)| val != &&' ')
        .for_each(|(i, val)| state[i].push(*val))
    );

    state
}

fn parse_moves(moves_string: &str) -> Vec<(usize, usize, usize)> {
    moves_string
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|split| [split[1], split[3], split[5]])
        .map(|nums| nums.map(|n| n.parse::<usize>().unwrap()))
        .map(|nums| (nums[0], nums[1] - 1, nums[2] - 1))
        .collect::<Vec<(usize, usize, usize)>>()
}
