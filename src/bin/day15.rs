use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;

struct Day15;

#[derive(Clone, Copy, Debug)]
struct Disk {
    id: usize,
    positions: usize,
    position_at_0: usize,
}

impl Disk {
    fn position_at(&self, time: usize) -> usize {
        (self.id + self.position_at_0 + time) % self.positions
    }
}

impl FromStr for Disk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).")
                    .unwrap();
        }

        let parse_int = |caps: &Captures, group: usize| {
            caps.get(group)
                .ok_or_else(|| anyhow::anyhow!("Getting group {} failed: {}", group, s))?
                .as_str()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Int parse error: {}", e))
        };

        if let Some(ref caps) = RE.captures(s) {
            let id = parse_int(caps, 1)?;
            let positions = parse_int(caps, 2)?;
            let position_at_0 = parse_int(caps, 3)?;
            Ok(Self {
                id,
                positions,
                position_at_0,
            })
        } else {
            Err(anyhow::anyhow!("Error parsing: {}", s))
        }
    }
}

impl Problem for Day15 {
    type Input = VecFromLines<Disk>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        // brute force
        let (first, rest) = input.split_first().expect("there should some disks");
        for time in 0.. {
            let expected_position = first.position_at(time);
            if rest
                .iter()
                .all(|disk| disk.position_at(time) == expected_position)
            {
                return time;
            }
        }
        unreachable!("time should have been found");
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut disks = input.to_vec();
        disks.push(Disk {
            id: disks.len() + 1,
            positions: 11,
            position_at_0: 0,
        });
        Self::solve_part1(&disks)
    }
}

fn main() {
    solve::<Day15>(include_str!("../../inputs/day15.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    const SAMPLE: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.\nDisc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day15>(SAMPLE), 5);
    }
}
