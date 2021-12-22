#[derive(Debug)]
struct DeterministicDie {
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie { rolls: 0 }
    }
}

trait Roll {
    fn roll(&mut self) -> usize;
}

impl Roll for DeterministicDie {
    fn roll(&mut self) -> usize {
        self.rolls = self.rolls + 1;
        self.rolls % 100
    }
}

struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Player { position, score: 0 }
    }
    fn play(&mut self, die: &mut impl Roll) {
        let roll = (0..3).map(|_| die.roll()).sum::<usize>();
        self.position = (self.position + roll) % 10;
        if self.position == 0 {
            self.position = 10;
        }
        self.score += self.position;
    }
}

fn main() {
    let mut p1 = Player::new(4);
    let mut p2 = Player::new(10);
    let mut die = DeterministicDie::new();
    loop {
        p1.play(&mut die);
        if p1.score >= 1000 {
            println!(
                "Player 1 wins. X = {} * {} = {}",
                p2.score,
                die.rolls,
                p2.score * die.rolls
            );
            break;
        }
        p2.play(&mut die);
        if p2.score >= 1000 {
            println!(
                "Player 2 wins. X = {} * {} = {}",
                p1.score,
                die.rolls,
                p1.score * die.rolls
            );
            break;
        }
    }
}
