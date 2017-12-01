use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    input.pop();
    println!("Captcha for part one = {}", reverse_captcha(&input));
}

fn reverse_captcha(input: &String) -> u32 {
    let mut sum = 0;
    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        let n_minus_1 = input
            .chars()
            .nth(if i == 0 { input.len() - 1 } else { i - 1 })
            .unwrap()
            .to_digit(10)
            .unwrap();
        if n == n_minus_1 {
            sum += n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn part_one_test1() {
        assert_eq!(reverse_captcha(&String::from("1122")), 3);
    }

    #[test]
    fn part_one_test2() {
        assert_eq!(reverse_captcha(&String::from("1111")), 4);
    }

    #[test]
    fn part_one_test3() {
        assert_eq!(reverse_captcha(&String::from("1234")), 0);
    }

    #[test]
    fn part_one_test4() {
        assert_eq!(reverse_captcha(&String::from("91212129")), 9);
    }
}
