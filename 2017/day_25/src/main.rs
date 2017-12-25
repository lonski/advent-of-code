use std::collections::HashMap;

fn main() {
    let mut machine = TuringMachine::new();
    machine.run(12523873);
    let checksum = machine
        .tape
        .iter()
        .filter(|&(pos, val)| *val == true)
        .count();
    println!("{}", checksum);
}

struct TuringMachine {
    cursor: i64,
    tape: HashMap<i64, bool>,
    state: State,
}

impl TuringMachine {
    fn new() -> Self {
        TuringMachine {
            cursor: 0,
            tape: HashMap::new(),
            state: State::A,
        }
    }

    fn run(&mut self, times: usize) {
        (0..times).for_each(|_| {
            let current_value = *self.tape.get(&self.cursor).unwrap_or(&false);
            self.state = match self.state {
                State::A => {
                    self.write(true);
                    if !current_value {
                        self.cursor += 1;
                        State::B
                    } else {
                        self.cursor -= 1;
                        State::E
                    }
                }
                State::B => {
                    self.write(true);
                    if !current_value {
                        self.cursor += 1;
                        State::C
                    } else {
                        self.cursor += 1;
                        State::F
                    }
                }
                State::C => {
                    if !current_value {
                        self.write(true);
                        self.cursor -= 1;
                        State::D
                    } else {
                        self.write(false);
                        self.cursor += 1;
                        State::B
                    }
                }
                State::D => {
                    if !current_value {
                        self.write(true);
                        self.cursor += 1;
                        State::E
                    } else {
                        self.write(false);
                        self.cursor -= 1;
                        State::C
                    }
                }
                State::E => {
                    if !current_value {
                        self.write(true);
                        self.cursor -= 1;
                        State::A
                    } else {
                        self.write(false);
                        self.cursor += 1;
                        State::D
                    }
                }
                State::F => {
                    if !current_value {
                        self.write(true);
                        self.cursor += 1;
                        State::A
                    } else {
                        self.write(true);
                        self.cursor += 1;
                        State::C
                    }
                }
            };
        });
    }

    fn write(&mut self, val: bool) {
        self.tape.insert(self.cursor, val);
    }
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}
