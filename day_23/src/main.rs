use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut program = Program::new(&input);
    println!("Part one: {}", program.execute());
    println!("Part two: {}", part_two());
}

fn part_two() -> i64 {
    let mut d;
    let mut f;
    let mut g;
    let mut h = 0;
    let mut b = 109900;
    let c = 126900;
    loop {
        f = 1;
        d = 2;
        while d != b {
            if b % d == 0 {
                f = 0;
                break;
            }
            d += 1;
        }
        if f == 0 {
            h += 1;
        }
        g = b - c;
        if g == 0 {
            break;
        }
        b += 17;
    }
    h
}

struct Program {
    registers: HashMap<char, i64>,
    instructions: Vec<String>,
    pc: usize,
}

impl Program {
    fn new(instructions: &str) -> Self {
        Program {
            registers: HashMap::new(),
            instructions: instructions.lines().map(String::from).collect(),
            pc: 0,
        }
    }

    fn execute(&mut self) -> i64 {
        let mut muls = 0;

        loop {
            if self.pc >= self.instructions.len() {
                break;
            }
            let i: Vec<&str> = self.instructions[self.pc].split_whitespace().collect();
            let reg: char = i[1].chars().nth(0).unwrap();
            match i[0] {
                "set" => {
                    let val = self.value_of(i[2]);
                    self.registers.insert(reg, val);
                    self.pc += 1;
                }
                "add" => {
                    let to_add = self.value_of(i[2]);
                    let current_val = self.value_of(i[1]);
                    self.registers.insert(reg, current_val + to_add);
                    self.pc += 1;
                }
                "sub" => {
                    let to_sub = self.value_of(i[2]);
                    let current_val = self.value_of(i[1]);
                    self.registers.insert(reg, current_val - to_sub);
                    self.pc += 1;
                }
                "mul" => {
                    let multiplier = self.value_of(i[2]);
                    let reg_val = self.value_of(i[1]);
                    self.registers.insert(reg, reg_val * multiplier);
                    self.pc += 1;
                    muls += 1;
                }
                "mod" => {
                    let m = self.value_of(i[2]);
                    let reg_val = self.value_of(i[1]);
                    self.registers.insert(reg, reg_val % m);
                    self.pc += 1;
                }
                "jnz" => {
                    let jmp_val = self.value_of(i[2]);
                    let val = self.value_of(i[1]);
                    let pc_mod: i64 = if val != 0 { jmp_val } else { 1 };
                    self.pc = (self.pc as i64 + pc_mod) as usize;
                }
                _ => panic!("Unknown instruction {:?}", i),
            }
        }
        muls
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
