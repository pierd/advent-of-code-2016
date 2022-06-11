use aoc_helpers::scaffold::{solve, Problem};

struct Day16;

#[derive(Debug)]
enum BitSeq {
    Simple(Vec<bool>),
    Dragon(Box<BitSeq>),
    Trimmed(Box<BitSeq>, usize),
    Checksum(Box<BitSeq>),
}

impl BitSeq {
    fn new(bits: Vec<bool>) -> Self {
        Self::Simple(bits)
    }

    fn dragon(self) -> Self {
        Self::Dragon(Box::new(self))
    }

    fn trim(self, len: usize) -> Self {
        if self.len() == len {
            self
        } else {
            Self::Trimmed(Box::new(self), len)
        }
    }

    fn checksum(self) -> Result<Self, Self> {
        if self.len() % 2 == 0 {
            Ok(Self::Checksum(Box::new(self)))
        } else {
            Err(self)
        }
    }

    fn len(&self) -> usize {
        match self {
            BitSeq::Simple(v) => v.len(),
            BitSeq::Dragon(b) => b.len() * 2 + 1,
            BitSeq::Trimmed(_, l) => *l,
            BitSeq::Checksum(b) => {
                let l = b.len();
                debug_assert!(l % 2 == 0);
                l / 2
            }
        }
    }

    fn get(&self, idx: usize) -> bool {
        match self {
            BitSeq::Simple(v) => v[idx],
            BitSeq::Dragon(b) => {
                let len = b.len();
                match idx.cmp(&len) {
                    std::cmp::Ordering::Less => b.get(idx),
                    std::cmp::Ordering::Equal => false,
                    std::cmp::Ordering::Greater => {
                        let relative_idx = len - 1 - (idx - len - 1);
                        !b.get(relative_idx)
                    }
                }
            }
            BitSeq::Trimmed(b, l) => {
                assert!(idx < *l);
                b.get(idx)
            }
            BitSeq::Checksum(b) => b.get(idx * 2) == b.get(idx * 2 + 1),
        }
    }
}

struct BitSeqIterator {
    seq: BitSeq,
    idx: usize,
    len: usize,
}

impl Iterator for BitSeqIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        if idx < self.len {
            self.idx += 1;
            Some(self.seq.get(idx))
        } else {
            None
        }
    }
}

impl IntoIterator for BitSeq {
    type Item = bool;

    type IntoIter = BitSeqIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            len: self.len(),
            seq: self,
            idx: 0,
        }
    }
}

const DISK_SIZE: usize = 272;
const BIG_DISK_SIZE: usize = 35651584;

fn fill_checksum(bits: Vec<bool>, size: usize) -> String {
    let mut seq = BitSeq::new(bits);
    while seq.len() < size {
        seq = seq.dragon();
    }
    seq = seq.trim(size);
    while seq.len() % 2 == 0 {
        seq = seq.checksum().expect("should be even length");
    }
    seq.into_iter().map(|b| if b { '1' } else { '0' }).collect()
}

impl Problem for Day16 {
    type Input = String;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let bits = input.chars().map(|c| c == '1').collect::<Vec<_>>();
        fill_checksum(bits, DISK_SIZE)
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let bits = input.chars().map(|c| c == '1').collect::<Vec<_>>();
        fill_checksum(bits, BIG_DISK_SIZE)
    }
}

fn main() {
    solve::<Day16>(include_str!("../../inputs/day16.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            fill_checksum(vec![true, false, false, false, false], 20),
            "01100"
        );
    }
}
