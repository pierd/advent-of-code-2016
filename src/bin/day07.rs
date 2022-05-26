use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};

struct Day07;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SeqType {
    Normal,
    Hypernet,
}

impl SeqType {
    fn other(self) -> Self {
        match self {
            SeqType::Normal => SeqType::Hypernet,
            SeqType::Hypernet => SeqType::Normal,
        }
    }
}

impl Default for SeqType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug)]
struct Sequence {
    seq_type: SeqType,
    seq: Vec<u8>,
}

impl Sequence {
    fn has_abba(&self) -> bool {
        self.seq
            .windows(4)
            .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
    }

    fn iter_normalised_abas(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
        self.seq.windows(3).filter_map(|w| {
            if w[0] == w[2] && w[0] != w[1] {
                Some(match self.seq_type {
                    SeqType::Normal => (w[0], w[1]),
                    SeqType::Hypernet => (w[1], w[0]),
                })
            } else {
                None
            }
        })
    }
}

struct Ip(Vec<Sequence>);

impl Ip {
    fn supports_tls(&self) -> bool {
        self.0
            .iter()
            .any(|s| s.seq_type == SeqType::Normal && s.has_abba())
            && !self
                .0
                .iter()
                .any(|s| s.seq_type == SeqType::Hypernet && s.has_abba())
    }

    fn supports_ssl(&self) -> bool {
        let mut all_abas = HashMap::new();
        all_abas.insert(SeqType::Normal, HashSet::new());
        all_abas.insert(SeqType::Hypernet, HashSet::new());
        for seq in self.0.iter() {
            for aba in seq.iter_normalised_abas() {
                if all_abas
                    .get(&seq.seq_type.other())
                    .map(|set| set.contains(&aba))
                    .unwrap_or_default()
                {
                    return true;
                } else {
                    all_abas.entry(seq.seq_type).or_default().insert(aba);
                }
            }
        }
        false
    }
}

impl FromStr for Ip {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seqs = Vec::new();
        let mut seq_type = Default::default();
        let mut seq = Vec::new();
        for b in s.as_bytes() {
            match b {
                b'[' => {
                    seqs.push(Sequence { seq, seq_type });
                    seq = Vec::new();
                    seq_type = SeqType::Hypernet;
                }
                b']' => {
                    seqs.push(Sequence { seq, seq_type });
                    seq = Vec::new();
                    seq_type = SeqType::Normal;
                }
                _ => seq.push(*b),
            }
        }
        seqs.push(Sequence { seq, seq_type });
        Ok(Self(seqs))
    }
}

impl Problem for Day07 {
    type Input = VecFromLines<Ip>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().filter(|ip| ip.supports_tls()).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input.iter().filter(|ip| ip.supports_ssl()).count()
    }
}

fn main() {
    solve::<Day07>(include_str!("../../inputs/day07.txt"));
}
