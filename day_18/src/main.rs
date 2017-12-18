use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", execute(&input));
}

fn execute(instructions: &str) -> i64 {
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut sound = 0;
    let mut pc: usize = 0;
    let instructions: Vec<String> = instructions.lines().map(String::from).collect();

    loop {
        let i: Vec<&str> = instructions[pc].split_whitespace().collect();
        let reg: char = i[1].chars().nth(0).unwrap();
        match i[0] {
            "set" => {
                let val = value_of(i[2], &registers);
                registers.insert(reg, val);
                pc += 1;
            }
            "snd" => {
                sound = *registers.get(&reg).unwrap_or(&0);
                pc += 1;
            }
            "add" => {
                let to_add = value_of(i[2], &registers);
                let current_val = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg, current_val + to_add);
                pc += 1;
            }
            "mul" => {
                let multiplier = value_of(i[2], &registers);
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg, reg_val * multiplier);
                pc += 1;
            }
            "mod" => {
                let m = value_of(i[2], &registers);
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg, reg_val % m);
                pc += 1;
            }
            "rcv" => {
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                if reg_val != 0 {
                    return sound;
                }
                pc += 1;
            }
            "jgz" => {
                let jmp_val = value_of(i[2], &registers);
                let reg_val = *registers.get(&reg).unwrap_or(&0);
                let pc_mod: i64 = if reg_val > 0 { jmp_val } else { 1 };
                pc = (pc as i64 + pc_mod) as usize;
            }
            _ => panic!("Unknown instruction {:?}", i),
        }
    }
}

fn value_of(arg: &str, registers: &HashMap<char, i64>) -> i64 {
    match arg.parse::<i64>() {
        Ok(v) => v,
        Err(_) => {
            let reg = arg.chars().nth(0).unwrap();
            *registers.get(&reg).unwrap_or(&0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = String::from(
            "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2",
        );
        assert_eq!(execute(&input), 4);
    }
}
