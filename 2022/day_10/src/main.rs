use std::{env, fs};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Instruction {
    NOOP,
    ADDX { argument: i32 },
}

fn parse_instructions(input: String) -> Vec<(u32, Instruction)> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_instruction)
        .collect::<Vec<(u32, Instruction)>>()
}

fn parse_instruction(line: &str) -> (u32, Instruction) {
    let mut split = line.split(" ");
    let instr = split.next().unwrap();
    let arg = split.next();
    match instr {
        "noop" => (1, Instruction::NOOP),
        "addx" => (
            2,
            Instruction::ADDX {
                argument: arg.unwrap().parse::<i32>().unwrap(),
            },
        ),
        _ => panic!("Unhandled instruction {}", line),
    }
}

fn signal_strength(val_by_cycle: &Vec<i32>, cycle_filter: Vec<usize>) -> i32 {
    val_by_cycle
        .iter()
        .enumerate()
        .map(|(index, val)| (index + 1, val))
        .filter(|(cycle, _)| cycle_filter.contains(cycle))
        .map(|(cycle, &val)| val * cycle as i32)
        .sum::<i32>()
}

fn print_screen(sprite_pos_by_cycle: &Vec<i32>) {
    const ROW_WIDTH: usize = 40;
    sprite_pos_by_cycle
        .iter()
        .enumerate()
        .map(
            |(cycle, &sprite_pos)| match sprite_pos - (cycle % ROW_WIDTH) as i32 {
                -1..=1 => '⭐',
                _ => '🌲',
            },
        )
        .collect::<Vec<char>>()
        .chunks(ROW_WIDTH)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let instructions = parse_instructions(input);

    let mut sprite_pos: i32 = 1;
    let mut sprite_pos_by_cycle = Vec::new();
    let mut program_counter = 0;
    let mut instruction_cycles = 0;

    while program_counter < instructions.len() {
        sprite_pos_by_cycle.push(sprite_pos);

        instruction_cycles += 1;
        let instruction = instructions[program_counter];
        if instruction_cycles == instruction.0 {
            match instruction.1 {
                Instruction::ADDX { argument } => sprite_pos += argument,
                Instruction::NOOP => {}
            }
            program_counter += 1;
            instruction_cycles = 0;
        }
    }

    println!(
        "Signal strength: {}",
        signal_strength(&sprite_pos_by_cycle, vec![20, 60, 100, 140, 180, 220])
    );
    print_screen(&sprite_pos_by_cycle);
}
