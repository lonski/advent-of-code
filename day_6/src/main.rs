use std::collections::HashSet;

fn main() {
    let memory: Vec<u32> = vec![2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];
    let (part_one_cycles, memory_after_part_one) = redistribute_until_duplicated_state(memory);
    println!("Part one: {}", part_one_cycles);
    println!(
        "Part two: {}",
        redistribute_until_duplicated_state(memory_after_part_one).0
    );
}

fn redistribute_until_duplicated_state(mut memory: Vec<u32>) -> (u32, Vec<u32>) {
    let mut states = HashSet::new();
    let mut cycles = 0;
    while !states.contains(&memory) {
        states.insert(memory.clone());
        let buffer_idx = first_max_idx(&memory);
        let blocks = memory[buffer_idx];
        memory[buffer_idx] = 0;
        redistribute_blocks(buffer_idx, blocks, &mut memory);
        cycles += 1;
    }
    (cycles, memory)
}

fn first_max_idx(vec: &Vec<u32>) -> usize {
    vec.iter()
        .position(|v| v == vec.iter().max().unwrap())
        .unwrap()
}

fn redistribute_blocks(start_idx: usize, mut blocks: u32, memory: &mut Vec<u32>) {
    let mut idx = start_idx;
    while blocks > 0 {
        idx = (idx + 1) % memory.len();
        blocks -= 1;
        memory[idx] = memory[idx] + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_max() {
        assert_eq!(first_max_idx(&vec![80, 80, 80, 5, 4, 2, 3, 1]), 0);
        assert_eq!(first_max_idx(&vec![0, 1, 2, 3, 80, 80, 80, 5, 4]), 4);
    }

    #[test]
    fn test() {
        let mut input = vec![1, 1, 1, 1];
        let start_idx = 1;
        let blocks = 3;

        redistribute_blocks(start_idx, blocks, &mut input);

        assert_eq!(input, vec![2, 1, 2, 2]);
    }
}
