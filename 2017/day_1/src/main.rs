use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    input.pop();
    println!("Captcha = {:?}", reverse_captcha(&input));
}

fn reverse_captcha(input: &String) -> (u32, u32) {
    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    let size = input.len();
    let step = input.len() / 2;
    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        //Part one
        if n == nth(input, if i == 0 { size - 1 } else { i - 1 }) {
            sum_part_1 += n;
        }
        //Part two
        if n == nth(input, (i + step) % size) {
            sum_part_2 += n;
        }
    }
    (sum_part_1, sum_part_2)
}

fn nth(input: &String, idx: usize) -> u32 {
    input.chars().nth(idx).unwrap().to_digit(10).unwrap()
}

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn part_one() {
        assert_eq!(reverse_captcha(&String::from("1122")).0, 3);
        assert_eq!(reverse_captcha(&String::from("1111")).0, 4);
        assert_eq!(reverse_captcha(&String::from("1234")).0, 0);
        assert_eq!(reverse_captcha(&String::from("91212129")).0, 9);
    }

    #[test]
    fn part_two() {
        assert_eq!(reverse_captcha(&String::from("1212")).1, 6);
        assert_eq!(reverse_captcha(&String::from("1221")).1, 0);
        assert_eq!(reverse_captcha(&String::from("123425")).1, 4);
        assert_eq!(reverse_captcha(&String::from("123123")).1, 12);
        assert_eq!(reverse_captcha(&String::from("12131415")).1, 4);
    }

}
