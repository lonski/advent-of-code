use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut moves = String::new();

    file.read_to_string(&mut moves).unwrap();
    moves.pop();

    let state: Vec<char> = ('a' as u8..'p' as u8 + 1).map(|n| n as char).collect();

    println!(
        "Part one {:?}",
        dance(&state, &moves).iter().fold(String::new(), |acc, c| {
            format!("{}{}", acc, c)
        })
    );

    let mut states = Vec::new();
    let mut next_state = state.clone();
    let mut p2_dance = 0;
    for i in 0.. {
        next_state = dance(&next_state, &moves);
        states.push(next_state.clone());
        if next_state == state {
            p2_dance = (1000000000 % (i + 1)) - 1;
            break;
        }
    }
    println!(
        "Part two: {:?}",
        states[p2_dance].iter().fold(String::new(), |acc, c| {
            format!("{}{}", acc, c)
        })
    );
}

fn dance(init_state: &Vec<char>, moves: &str) -> Vec<char> {
    let mut state = init_state.clone();
    for m in moves.split(",") {
        state = match &m[0..1] {
            "s" => {
                let size = *&m[1..].parse::<usize>().unwrap();
                let mut new_state = state
                    .iter()
                    .skip(state.len() - size)
                    .take(size)
                    .cloned()
                    .collect::<Vec<char>>();
                new_state.extend(
                    state
                        .iter()
                        .cycle()
                        .take(state.len() - size)
                        .cloned()
                        .collect::<Vec<char>>(),
                );
                new_state
            }
            "x" => {
                let s = String::from(&m[1..]);
                let mut indexes = s.split("/");
                let idx1 = indexes.next().unwrap().parse::<usize>().unwrap();
                let idx2 = indexes.next().unwrap().parse::<usize>().unwrap();
                state.swap(idx1, idx2);
                state
            }
            "p" => {
                let s = String::from(&m[1..]);
                let mut indexes = s.split("/");
                let c1 = indexes.next().unwrap().chars().nth(0).unwrap();
                let c2 = indexes.next().unwrap().chars().nth(0).unwrap();
                let mut idx1 = 666;
                let mut idx2 = 666;
                for (i, c) in state.iter().enumerate() {
                    if *c == c1 {
                        idx1 = i;
                    }
                    if *c == c2 {
                        idx2 = i;
                    }
                }
                state.swap(idx1, idx2);
                state
            }
            _ => panic!("Unrecognized move"),
        };
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let state = vec!['a', 'b', 'c', 'd', 'e'];
        let moves = "s1,x3/4,pe/b";

        let new_state = dance(&state, moves);
        assert_eq!(new_state, vec!['b', 'a', 'e', 'd', 'c']);
    }
}
