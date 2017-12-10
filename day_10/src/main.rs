fn main() {
    let input = "34,88,2,222,254,93,150,0,199,255,39,32,137,136,1,167";
    println!("Part one: {}", part_one(input, create_byte_list()));
    println!("Part two: {}", knot_hash(input));
}

fn part_one(input: &str, list: Vec<u8>) -> u32 {
    let lengths: Vec<u8> = input.split(",").filter_map(|c| c.parse().ok()).collect();
    let list = knot_round(&lengths, list, 1);
    list[0] as u32 * list[1] as u32
}

fn knot_hash(input: &str) -> String {
    let mut lengths: Vec<u8> = input.chars().map(|c| c as u8).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    knot_round(&lengths, create_byte_list(), 64)
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, i| acc ^ i))
        .map(|c| format!("{:02X}", c))
        .map(|c| c.to_lowercase())
        .collect()
}

fn knot_round(lengths: &Vec<u8>, mut list: Vec<u8>, rounds: u32) -> Vec<u8> {
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;
    for _ in 0..rounds {
        for length in lengths {
            reverse(&mut list, current_pos, *length as usize);
            current_pos = (current_pos + *length as usize + skip_size) % list.len();
            skip_size += 1;
        }
    }
    list
}

fn create_byte_list() -> Vec<u8> {
    let mut list: Vec<u8> = (0..255).collect();
    list.push(255);
    list
}

fn reverse(list: &mut Vec<u8>, mut start: usize, length: usize) {
    let list_size: usize = list.len();
    let mut end = start + length - 1;
    while end as i32 - start as i32 > 0 {
        let tmp: u8 = list[end % list_size];
        list[end % list_size] = list[start % list_size];
        list[start % list_size] = tmp;
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_examples_test() {
        assert_eq!(
            knot_hash("1,2,3"),
            String::from("3efbe78a8d82f29979031a4aa0b16a9d")
        );
        assert_eq!(
            knot_hash("1,2,4"),
            String::from("63960835bcdc130f0b66d7ff4f6a5a8e")
        );
        assert_eq!(
            knot_hash("AoC 2017"),
            String::from("33efeb34ea91902bb2f59c9920caa6cd")
        );
        assert_eq!(
            knot_hash(""),
            String::from("a2582a3a0e66e6e86e3812dcb672a272")
        );
    }

    #[test]
    fn part_one_examples_test() {
        assert_eq!(part_one("3,4,1,5", vec![0, 1, 2, 3, 4]), 12);
    }

    #[test]
    fn reverse_test() {
        let mut vec = vec![1, 2, 3, 4];
        reverse(&mut vec, 0, 4);
        assert_eq!(&vec, &vec![4, 3, 2, 1]);
        reverse(&mut vec, 0, 3);
        assert_eq!(&vec, &vec![2, 3, 4, 1]);
        reverse(&mut vec, 1, 2);
        assert_eq!(&vec, &vec![2, 4, 3, 1]);
        reverse(&mut vec, 2, 3);
        assert_eq!(&vec, &vec![3, 4, 2, 1]);
    }

}
