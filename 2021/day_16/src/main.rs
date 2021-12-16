use std::env;
use std::fs;

#[derive(Debug)]
struct Packet {
    version: u32,
    id: u32,
    value: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn calculate(&self) -> u64 {
        match self.id {
            0 => self.sub_packets.iter().map(|p| p.calculate()).sum(),
            1 => self
                .sub_packets
                .iter()
                .map(|p| p.calculate())
                .fold(1, |acc, v| acc * v),
            2 => self
                .sub_packets
                .iter()
                .map(|p| p.calculate())
                .min()
                .unwrap(),
            3 => self
                .sub_packets
                .iter()
                .map(|p| p.calculate())
                .max()
                .unwrap(),
            4 => self.value,
            5 => match self.sub_packets[0].calculate() > self.sub_packets[1].calculate() {
                true => 1,
                false => 0,
            },
            6 => match self.sub_packets[0].calculate() < self.sub_packets[1].calculate() {
                true => 1,
                false => 0,
            },
            7 => match self.sub_packets[0].calculate() == self.sub_packets[1].calculate() {
                true => 1,
                false => 0,
            },
            _ => panic!("Unchandled id: {}", self.id),
        }
    }
}

fn to_binary<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_transmission<'a>(input: String) -> String {
    input.chars().map(to_binary).collect()
}

fn parse_packet<'a>(input: &'a str) -> (Packet, &'a str) {
    let version = u32::from_str_radix(&input[0..3], 2).unwrap();
    let id = u32::from_str_radix(&input[3..6], 2).unwrap();
    let input = &input[6..];

    //value packet
    if id == 4 {
        let mut ptr = 0;
        let mut value = String::from("");
        for chunk in input.chars().collect::<Vec<char>>().chunks(5) {
            value.push_str(&chunk[1..].iter().collect::<String>());
            ptr += chunk.len();
            if chunk[0] == '0' {
                break;
            }
        }
        let value = u64::from_str_radix(&value, 2).unwrap();
        return (
            Packet {
                version,
                id,
                value,
                sub_packets: vec![],
            },
            &input[ptr..],
        );
    }

    //operator packet
    let len_id = input.chars().next().unwrap();
    let len = if len_id == '0' { 15 } else { 11 };
    let number = usize::from_str_radix(&input[1..len + 1], 2).unwrap();
    let input = &input[len + 1..];
    let mut sub_packets = Vec::new();

    // fixed length packet
    if len_id == '0' {
        let mut raw_subpackets = &input[0..number];
        let mut rem = raw_subpackets.len();
        while rem > 6 {
            let (sub_packet, remaining) = parse_packet(raw_subpackets);
            sub_packets.push(sub_packet);
            raw_subpackets = remaining;
            rem = remaining.len();
        }

        return (
            Packet {
                version,
                id,
                value: 0,
                sub_packets,
            },
            &input[number..],
        );
    }

    // fixed subpacket count packet
    let mut raw_subpackets = input;
    for _ in 0..number {
        let (sub_packet, remaining) = parse_packet(raw_subpackets);
        sub_packets.push(sub_packet);
        raw_subpackets = remaining;
    }

    (
        Packet {
            version,
            id,
            value: 0,
            sub_packets,
        },
        &raw_subpackets,
    )
}

fn sum_version_numbers(packet: &Packet) -> u32 {
    packet.version
        + packet
            .sub_packets
            .iter()
            .map(|p| sum_version_numbers(p))
            .sum::<u32>()
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let input = fs::read_to_string(filename).unwrap();
    let binary_transmission = parse_transmission(input);
    let (packet, _) = parse_packet(&binary_transmission);

    println!("Part 1: {}", sum_version_numbers(&packet));
    println!("Part 2: {}", packet.calculate());
}
