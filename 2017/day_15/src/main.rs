fn main() {
    println!("Part one: {}", part_one(873, 583));
    println!("Part two: {}", part_two(873, 583));
}

fn part_one(g1_val: u64, g2_val: u64) -> usize {
    let mut g1 = Generator {
        value: g1_val,
        factor: 16807,
        mul: 1,
    };
    let mut g2 = Generator {
        value: g2_val,
        factor: 48271,
        mul: 1,
    };
    (0..40000001)
        .map(|_| (g1.next(), g2.next()))
        .filter(|&(v1, v2)| v1 & 0xffff == v2 & 0xffff)
        .count()
}

fn part_two(g1_val: u64, g2_val: u64) -> usize {
    let mut g1 = Generator {
        value: g1_val,
        factor: 16807,
        mul: 4,
    };
    let mut g2 = Generator {
        value: g2_val,
        factor: 48271,
        mul: 8,
    };
    (0..5000001)
        .map(|_| (g1.next(), g2.next()))
        .filter(|&(v1, v2)| v1 & 0xffff == v2 & 0xffff)
        .count()
}

struct Generator {
    value: u64,
    factor: u64,
    mul: u64,
}

impl Generator {
    fn next(&mut self) -> u64 {
        self.value = (self.value * self.factor) % 2147483647;
        while self.mul != 1 && self.value % self.mul != 0 {
            self.value = (self.value * self.factor) % 2147483647;
        }
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_next_value() {
        let mut g1 = Generator {
            value: 65,
            factor: 16807,
            mul: 1,
        };
        assert_eq!(g1.next(), 1092455);
        assert_eq!(g1.next(), 1181022009);
        assert_eq!(g1.next(), 245556042);
        assert_eq!(g1.next(), 1744312007);
        assert_eq!(g1.next(), 1352636452);

        let mut g2 = Generator {
            value: 8921,
            factor: 48271,
            mul: 1,
        };
        assert_eq!(g2.next(), 430625591);
        assert_eq!(g2.next(), 1233683848);
        assert_eq!(g2.next(), 1431495498);
        assert_eq!(g2.next(), 137874439);
        assert_eq!(g2.next(), 285222916);
    }

    #[test]
    fn should_extract_16_lowest_bits() {
        let mut g1 = Generator {
            value: 65,
            factor: 16807,
            mul: 1,
        };
        g1.next();
        assert_eq!(g1.lowest_16bits(), String::from("1010101101100111"));
        g1.next();
        assert_eq!(g1.lowest_16bits(), String::from("1111011100111001"));
        g1.next();
        assert_eq!(g1.lowest_16bits(), String::from("1110001101001010"));
        g1.next();
        assert_eq!(g1.lowest_16bits(), String::from("0001011011000111"));
        g1.next();
        assert_eq!(g1.lowest_16bits(), String::from("1001100000100100"));
    }
}
