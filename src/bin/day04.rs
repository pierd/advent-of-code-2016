use std::{collections::HashMap, str::FromStr};

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};

struct Day04;

struct Room {
    name: String,
    sector_id: usize,
    checksum: String,
}

fn shift_chr(c: char, times: usize) -> char {
    if ('a'..='z').contains(&c) {
        ((c as usize - 'a' as usize + times) % ('z' as usize - 'a' as usize + 1) + 'a' as usize)
            as u8 as char
    } else {
        ' '
    }
}

impl Room {
    fn calculate_checksum(&self) -> String {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in self.name.chars().filter(char::is_ascii_alphabetic) {
            *counts.entry(c).or_default() += 1
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|&(chr, count)| {
            (
                -isize::try_from(count).expect("count should be small enough"),
                chr,
            )
        });
        counts.into_iter().take(5).map(|(chr, _)| chr).collect()
    }

    fn is_real(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn decrypt_name(&self) -> String {
        self.name
            .chars()
            .map(|c| shift_chr(c, self.sector_id))
            .collect()
    }
}

impl FromStr for Room {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_suffix(']')
            .ok_or_else(|| anyhow::anyhow!("Unexpected final character"))?;
        let (s, checksum) = s.split_at(s.len() - 5);
        let s = s
            .strip_suffix('[')
            .ok_or_else(|| anyhow::anyhow!("Malformed checksum"))?;
        let (s, sector_id) = s
            .rsplit_once('-')
            .ok_or_else(|| anyhow::anyhow!("Failed to find sector id"))?;

        Ok(Self {
            name: s.to_owned(),
            sector_id: sector_id
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Failed to parse sector id: {}", e))?,
            checksum: checksum.to_owned(),
        })
    }
}

impl Problem for Day04 {
    type Input = VecFromLines<Room>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .iter()
            .filter_map(|r| if r.is_real() { Some(r.sector_id) } else { None })
            .sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input
            .iter()
            .find(|r| r.is_real() && r.decrypt_name().starts_with("north"))
            .expect("there should be a room")
            .sector_id
    }
}

fn main() {
    solve::<Day04>(include_str!("../../inputs/day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "aaaaa-bbb-z-y-x-123[abxyz]";
    const SAMPLE2: &str = "a-b-c-d-e-f-g-h-987[abcde]";
    const SAMPLE3: &str = "not-a-real-room-404[oarel]";
    const SAMPLE4: &str = "totally-real-room-200[decoy]";

    #[test]
    fn test_sample() {
        let s = SAMPLE1.parse::<Room>().unwrap();
        assert_eq!(s.name, "aaaaa-bbb-z-y-x");
        assert_eq!(s.checksum, "abxyz");
        assert_eq!(s.sector_id, 123);
        assert!(s.is_real());

        assert!(SAMPLE2.parse::<Room>().unwrap().is_real());
        assert!(SAMPLE3.parse::<Room>().unwrap().is_real());
        assert!(!SAMPLE4.parse::<Room>().unwrap().is_real());
    }

    #[test]
    fn test_sample2() {
        let r = "qzmt-zixmtkozy-ivhz-343[abcde]".parse::<Room>().unwrap();
        assert_eq!(r.decrypt_name(), "very encrypted name");
    }
}
