#[derive(Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: fn(old: u128) -> u128,
    test: fn(worry_level: u128) -> u128,
    test_divisor: u128,
    inspected_items: u128,
}

fn monkey_business(monkeys: &Vec<Monkey>) -> u128 {
    let mut inspected_items = monkeys
        .iter()
        .map(|m| m.inspected_items)
        .collect::<Vec<u128>>();
    inspected_items.sort();
    inspected_items.reverse();

    inspected_items[0] * inspected_items[1]
}

fn monkey_round(monkeys: &mut Vec<Monkey>, div_by: u128) {
    // least common multiplier of all monkeys test divisor
    // failed to figure it by myself, found the hint on AoC reddit
    let monkey_tests_lcm: u128 = monkeys.iter().map(|m| m.test_divisor).product();

    for monkey_id in 0..monkeys.len() {
        let monkey = &mut monkeys[monkey_id];
        let processed_items = monkey
            .items
            .iter()
            .map(|&item| {
                let worry_level = (monkey.operation)(item) % monkey_tests_lcm / div_by;
                let target_monkey = (monkey.test)(worry_level);
                monkey.inspected_items += 1;
                return (worry_level, target_monkey);
            })
            .collect::<Vec<(u128, u128)>>();

        monkey.items.clear();

        processed_items
            .iter()
            .for_each(|&(item, target)| monkeys[target as usize].items.push(item));
    }
}

fn run(monkeys: &Vec<Monkey>, rounds: u32, div_by: u32) -> u128 {
    let mut monkeys_cloned = monkeys
        .iter()
        .clone()
        .map(|m| m.clone())
        .collect::<Vec<Monkey>>();
    for _ in 0..rounds {
        monkey_round(&mut monkeys_cloned, div_by as u128);
    }
    monkey_business(&monkeys_cloned)
}

fn main() {
    let monkeys = vec![
        Monkey {
            items: vec![85, 79, 63, 72],
            operation: |old| old * 17,
            test: |new| if new % 2 == 0 { 2 } else { 6 },
            test_divisor: 2,
            inspected_items: 0,
        },
        Monkey {
            items: vec![53, 94, 65, 81, 93, 73, 57, 92],
            operation: |old| old * old,
            test: |new| if new % 7 == 0 { 0 } else { 2 },
            test_divisor: 7,
            inspected_items: 0,
        },
        Monkey {
            items: vec![62, 63],
            operation: |old| old + 7,
            test: |new| if new % 13 == 0 { 7 } else { 6 },
            test_divisor: 13,
            inspected_items: 0,
        },
        Monkey {
            items: vec![57, 92, 56],
            operation: |old| old + 4,
            test: |new| if new % 5 == 0 { 4 } else { 5 },
            test_divisor: 5,
            inspected_items: 0,
        },
        Monkey {
            items: vec![67],
            operation: |old| old + 5,
            test: |new| if new % 3 == 0 { 1 } else { 5 },
            test_divisor: 3,
            inspected_items: 0,
        },
        Monkey {
            items: vec![85, 56, 66, 72, 57, 99],
            operation: |old| old + 6,
            test: |new| if new % 19 == 0 { 1 } else { 0 },
            test_divisor: 19,
            inspected_items: 0,
        },
        Monkey {
            items: vec![86, 65, 98, 97, 69],
            operation: |old| old * 13,
            test: |new| if new % 11 == 0 { 3 } else { 7 },
            test_divisor: 11,
            inspected_items: 0,
        },
        Monkey {
            items: vec![87, 68, 92, 66, 91, 50, 68],
            operation: |old| old + 2,
            test: |new| if new % 17 == 0 { 4 } else { 3 },
            test_divisor: 17,
            inspected_items: 0,
        },
    ];

    println!("Monkey business 1: {}", run(&monkeys, 20, 3));
    println!("Monkey business 2: {}", run(&monkeys, 10000, 1));
}
