use aoc_helpers::scaffold::{solve, Problem};
use rayon::prelude::*;

struct Day05;

const SEARCH_BATCH_SIZE: u32 = 0x10000;

fn find_next_digest_with_five_zeroes(mut start: u32, prefix: &str) -> Option<(u32, [u8; 16])> {
    loop {
        let result = (start..(start + SEARCH_BATCH_SIZE))
            .into_par_iter()
            .map(|n| -> (u32, [u8; 16]) { (n, md5::compute(format!("{prefix}{n}")).into()) })
            .find_first(|(_, d)| d[0..2] == [0, 0] && d[2] < 0x10);
        if result.is_some() {
            return result;
        } else {
            start += SEARCH_BATCH_SIZE;
        }
    }
}

impl Problem for Day05 {
    type Input = String;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut result = String::new();
        let mut n = 0;
        while result.len() < 8 {
            if let Some((found_n, digest)) = find_next_digest_with_five_zeroes(n, input) {
                let third_byte = format!("{:x}", digest[2]);
                result.push(
                    third_byte
                        .chars()
                        .next()
                        .expect("digest should be long enough"),
                );
                n = found_n + 1;
            } else {
                panic!("solution not found");
            }
        }
        result
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut result = [None; 8];
        let mut missing = 8;
        let mut n = 0;
        while missing != 0 {
            if let Some((found_n, digest)) = find_next_digest_with_five_zeroes(n, input) {
                let idx = digest[2] as usize;
                let chr = format!("{:x}", digest[3] >> 4)
                    .chars()
                    .next()
                    .expect("digest should be long enough");
                if idx < result.len() && result[idx].is_none() {
                    result[idx] = Some(chr);
                    missing -= 1;
                }
                n = found_n + 1;
            } else {
                panic!("solution not found");
            }
        }
        result
            .into_iter()
            .collect::<Option<String>>()
            .expect("all digits should be found")
    }
}

fn main() {
    solve::<Day05>(include_str!("../../inputs/day05.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "abc";

    #[test]
    fn test_hashing() {
        let digest: [u8; 16] = md5::compute("abc3231929").into();
        assert_eq!(digest[0..2], [0, 0]);
        assert!(digest[2] < 0x10);

        let five_zeros = find_next_digest_with_five_zeroes(3231929, "abc").unwrap();
        assert_eq!(five_zeros.0, 3231929);
    }

    #[test]
    #[ignore = "slow"]
    fn test_sample() {
        assert_eq!(solve_part1::<Day05>(SAMPLE), "18f47a30");
        assert_eq!(solve_part2::<Day05>(SAMPLE), "05ace8e3");
    }
}
