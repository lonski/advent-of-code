use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Wins {
    p1: i64,
    p2: i64,
}

impl Wins {
    fn add(&mut self, other: &Wins) {
        self.p1 += other.p1;
        self.p2 += other.p2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Self {
        Player { pos, score: 0 }
    }

    fn play(&mut self, steps: u32) {
        self.pos = (self.pos + steps) % 10;
        if self.pos == 0 {
            self.pos = 10;
        }
        self.score += self.pos;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    player1: Player,
    player2: Player,
    is_p1_turn: bool,
}

fn get_3d3_rolls() -> Vec<u32> {
    vec![1, 2, 3, 1, 2, 3, 1, 2, 3]
        .iter()
        .combinations(3)
        .unique()
        .map(|c| c.into_iter().sum::<u32>())
        .collect()
}

fn next_turn(state: State, wins_per_state: &mut HashMap<State, Wins>) -> Wins {
    if state.player1.score >= 21 {
        return Wins { p1: 1, p2: 0 };
    }
    if state.player2.score >= 21 {
        return Wins { p1: 0, p2: 1 };
    }

    let mut total_wins = Wins { p1: 0, p2: 0 };

    for roll in get_3d3_rolls() {
        let mut next_state = state;

        next_state.is_p1_turn = !state.is_p1_turn;
        match state.is_p1_turn {
            true => next_state.player1.play(roll as u32),
            false => next_state.player2.play(roll as u32),
        }

        let wins: Wins = match wins_per_state.get(&next_state) {
            Some(&wins) => wins,
            None => {
                let wins = next_turn(next_state, wins_per_state);
                wins_per_state.insert(next_state, wins);
                wins
            }
        };
        total_wins.add(&wins);
    }

    total_wins
}

fn main() {
    let wins = next_turn(
        State {
            player1: Player::new(4),
            player2: Player::new(10),
            is_p1_turn: true,
        },
        &mut HashMap::new(),
    );
    println!("{:?}", wins.p1.max(wins.p2));
}
