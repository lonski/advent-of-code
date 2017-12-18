use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let (tx, rx) = channel::<i64>();
    let (_, rx1) = channel::<i64>();
    let mut program = Program::new(0, &input, tx, rx1);
    println!("Part one: {}", program.execute().0);
}

fn part_two(input: &str) {
    let (tx0, rx0) = channel::<i64>();
    let (tx1, rx1) = channel::<i64>();

    let mut p0 = Program::new(0, input, tx0, rx1);
    let mut p1 = Program::new(1, input, tx1, rx0);

    let p0_handle = thread::spawn(move || { p0.execute(); });
    let p1_handle = thread::spawn(move || {
        println!("Part two: {}", p1.execute().1);
    });

    p0_handle.join().unwrap();
    p1_handle.join().unwrap();
}

struct Program {
    registers: HashMap<char, i64>,
    instructions: Vec<String>,
    pc: usize,
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl Program {
    fn new(p_id: i64, instructions: &str, tx: Sender<i64>, rx: Receiver<i64>) -> Self {
        let mut registers: HashMap<char, i64> = HashMap::new();
        registers.insert('p', p_id);
        Program {
            registers: registers,
            instructions: instructions.lines().map(String::from).collect(),
            pc: 0,
            tx: tx,
            rx: rx,
        }
    }

    fn execute(&mut self) -> (i64, i64) {
        let mut cnt: i64 = 0;
        let mut sent_value = 0;

        loop {
            let i: Vec<&str> = self.instructions[self.pc].split_whitespace().collect();
            let reg: char = i[1].chars().nth(0).unwrap();
            match i[0] {
                "set" => {
                    let val = self.value_of(i[2]);
                    self.registers.insert(reg, val);
                    self.pc += 1;
                }
                "snd" => {
                    sent_value = self.value_of(i[1]);
                    self.tx.send(sent_value).unwrap();
                    cnt += 1;
                    self.pc += 1;
                }
                "add" => {
                    let to_add = self.value_of(i[2]);
                    let current_val = self.value_of(i[1]);
                    self.registers.insert(reg, current_val + to_add);
                    self.pc += 1;
                }
                "mul" => {
                    let multiplier = self.value_of(i[2]);
                    let reg_val = self.value_of(i[1]);
                    self.registers.insert(reg, reg_val * multiplier);
                    self.pc += 1;
                }
                "mod" => {
                    let m = self.value_of(i[2]);
                    let reg_val = self.value_of(i[1]);
                    self.registers.insert(reg, reg_val % m);
                    self.pc += 1;
                }
                "rcv" => {
                    match self.rx.recv_timeout(Duration::from_millis(100)) {
                        Ok(val) => {
                            self.registers.insert(reg, val);
                            self.pc += 1;
                        }
                        Err(_) => {
                            return (sent_value, cnt);
                        }
                    }
                }
                "jgz" => {
                    let jmp_val = self.value_of(i[2]);
                    let reg_val = self.value_of(i[1]);
                    let pc_mod: i64 = if reg_val > 0 { jmp_val } else { 1 };
                    self.pc = (self.pc as i64 + pc_mod) as usize;
                }
                _ => panic!("Unknown instruction {:?}", i),
            }
        }
    }

    fn value_of(&self, arg: &str) -> i64 {
        match arg.parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                let reg = arg.chars().nth(0).unwrap();
                *self.registers.get(&reg).unwrap_or(&0)
            }
        }
    }
}
