extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use std::cmp;

type Registers = HashMap<String, i32>;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("(Part one, Part two): {:?}", solve(&input));
}

fn solve(input: &String) -> (i32, i32) {
    let mut registers: Registers = Registers::new();
    let instructions: Vec<Instruction> = input.lines().map(Instruction::new).collect();
    let mut max: i32 = 0;
    for i in instructions {
        if (*i.condition)(&registers) {
            let current_val: i32 = *registers.get(&i.register).unwrap_or(&0);
            let new_val = match i.operation {
                Operation::Inc(val) => {
                    let new_v = current_val + val;
                    registers.insert(i.register.clone(), new_v);
                    new_v
                }
                Operation::Dec(val) => {
                    let new_v = current_val - val;
                    registers.insert(i.register.clone(), new_v);
                    new_v
                }
            };
            max = cmp::max(max, new_val);
        }
    }
    (*registers.iter().max_by_key(|&(_, v)| v).unwrap().1, max)
}

struct Instruction {
    register: String,
    operation: Operation,
    condition: Box<Fn(&Registers) -> bool>,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let captures = Regex::new(
            r"^(\w+) (inc|dec) (-?\d+) if (\w+) (>|<|>=|<=|!=|==) (-?\d+)",
        ).unwrap()
            .captures(line)
            .unwrap();
        let operation_val = captures[3].parse::<i32>().unwrap();
        let operation = match &captures[2] {
            "inc" => Operation::Inc(operation_val),
            _ => Operation::Dec(operation_val),
        };
        let cond_val = captures[6].parse::<i32>().unwrap();
        let cond_reg = String::from(&captures[4]);
        let cond: Box<Fn(&Registers) -> bool> = match &captures[5] {
            ">" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) > cond_val),
            "<" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) < cond_val),
            ">=" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) >= cond_val),
            "<=" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) <= cond_val),
            "!=" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) != cond_val),
            "==" => Box::new(move |r| *r.get(&cond_reg).unwrap_or(&0) == cond_val),
            _ => panic!("Not supported operator"),
        };
        Instruction {
            register: String::from(&captures[1]),
            operation: operation,
            condition: cond,
        }
    }
}

enum Operation {
    Inc(i32),
    Dec(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = String::new();
        input.push_str("b inc 5 if a > 1\n");
        input.push_str("a inc 1 if b < 5\n");
        input.push_str("c dec -10 if a >= 1\n");
        input.push_str("c inc -20 if c == 10\n");

        assert_eq!(solve(&input), (1, 10));
    }
}
