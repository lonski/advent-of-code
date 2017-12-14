pub fn hash(input: &str) -> String {
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
