use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Bit {
    Undecided,
    Zero,
    One,
}

#[derive(Clone, Debug, Default)]
struct BitVec {
    bits: Vec<Bit>,
}

impl BitVec {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_triple(&mut self, index: usize, triple: u8) -> bool {
        if index + 2 >= self.bits.len() {
            self.bits.resize(index + 3, Bit::Undecided);
        }
        for i in 0..3 {
            let new_bit = if ((triple >> i) & 1) == 1 {
                Bit::One
            } else {
                Bit::Zero
            };
            let old_bit = self.bits[index + i];
            if old_bit != Bit::Undecided && old_bit != new_bit {
                return false;
            }
        }
        for i in 0..3 {
            let new_bit = if ((triple >> i) & 1) == 1 {
                Bit::One
            } else {
                Bit::Zero
            };
            self.bits[index + i] = new_bit;
        }
        true
    }

    pub fn finalize(self, index: usize) -> Option<u64> {
        for i in index..self.bits.len() {
            if self.bits[i] == Bit::One {
                return None;
            }
        }
        if self.bits[(index - 3)..index]
            .iter()
            .all(|&bit| bit == Bit::Zero)
        {
            return None;
        }

        Some(
            self.bits
                .iter()
                .enumerate()
                .map(|(i, &bit)| if bit == Bit::One { 1 << i } else { 0 })
                .sum(),
        )
    }
}

impl Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .bits
            .iter()
            .map(|bit| match bit {
                Bit::Undecided => 'X',
                Bit::Zero => '0',
                Bit::One => '1',
            })
            .rev()
            .collect();
        write!(f, "{}", s)
    }
}

fn search(bits: BitVec, offset: usize, goal: &[u8]) -> Option<u64> {
    let Some((&next, goal_rest)) = goal.split_first() else {
        return bits.finalize(offset);
    };
    // while a > 0:
    //   b = a % 8
    //   out <- (b ^ (a >> (b ^ 4))) % 8
    //   a /= 8
    for b in 0..8 {
        let mut new_bits = bits.clone();
        let shift = (b ^ 4) as usize;
        if new_bits.set_triple(offset, b) && new_bits.set_triple(offset + shift, next ^ b) {
            if let Some(a) = search(new_bits, offset + 3, goal_rest) {
                return Some(a);
            }
        }
    }
    None
}

fn main() {
    let goal: Vec<u8> = vec![2, 4, 1, 4, 7, 5, 4, 1, 1, 4, 5, 5, 0, 3, 3, 0];
    println!("{}", search(BitVec::new(), 0, &goal).unwrap());
}
