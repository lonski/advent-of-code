use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("(Part one, Part two): {:?}", solve(&input));
}

fn solve(input: &String) -> (u32, u32) {
    let mut level = 0;
    let mut score = 0;
    let mut garbage = 0;
    let mut stack: Vec<char> = vec!['\0'];
    for c in input.chars() {
        match stack[stack.len() - 1] {
            '!' => {
                stack.pop();
            }
            '<' => {
                match c {
                    '>' => {
                        stack.pop();
                    }
                    '!' => stack.push('!'),
                    _ => garbage += 1,
                };
            }
            _ => {
                match c {
                    '{' => {
                        stack.push('{');
                        level += 1;
                        score += level;
                    }
                    '}' => {
                        stack.pop();
                        level -= 1;
                    }
                    '!' => stack.push('!'),
                    '<' => stack.push('<'),
                    _ => (),
                }
            }
        }
    }
    (score, garbage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_test() {
        assert_eq!(solve(&String::from("{}")), (1, 0));
        assert_eq!(solve(&String::from("{!{!}}")), (1, 0));
        assert_eq!(solve(&String::from("{{{}}}")), (6, 0));
        assert_eq!(solve(&String::from("{{},{}}")), (5, 0));
        assert_eq!(solve(&String::from("{{{},{},{{}}}}")), (16, 0));
        assert_eq!(solve(&String::from("{<a>,<a>,<a>,<a>}")), (1, 4));
        assert_eq!(
            solve(&String::from("{{<ab>},{<ab>},{<ab>},{<ab>}}")),
            (9, 8)
        );
        assert_eq!(
            solve(&String::from("{{<!!>},{<!!>},{<!!>},{<!!>}}")),
            (9, 0)
        );
        assert_eq!(
            solve(&String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}")),
            (3, 17)
        );
    }
}
