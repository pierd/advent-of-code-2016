use std::{collections::HashMap, hash::Hash};

use aoc_helpers::scaffold::{solve, Problem, RowsOfChars};

struct Day06;

fn most_common<I: Iterator>(iter: I) -> Option<I::Item>
where
    I::Item: Eq + Hash,
{
    let mut counts: HashMap<I::Item, usize> = HashMap::new();
    for item in iter {
        *counts.entry(item).or_default() += 1;
    }
    let max_count = counts.iter().map(|(_, count)| *count).max();
    max_count.and_then(|m| counts.into_iter().find(|(_, v)| *v == m).map(|(k, _)| k))
}

fn least_common<I: Iterator>(iter: I) -> Option<I::Item>
where
    I::Item: Eq + Hash,
{
    let mut counts: HashMap<I::Item, usize> = HashMap::new();
    for item in iter {
        *counts.entry(item).or_default() += 1;
    }
    let min_count = counts.iter().map(|(_, count)| *count).min();
    min_count.and_then(|m| counts.into_iter().find(|(_, v)| *v == m).map(|(k, _)| k))
}

impl Problem for Day06 {
    type Input = RowsOfChars<char>;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        (0..input[0].len())
            .map(|idx| most_common(input.iter().map(|s| s[idx])))
            .collect::<Option<String>>()
            .expect("there should be a solution")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        (0..input[0].len())
            .map(|idx| least_common(input.iter().map(|s| s[idx])))
            .collect::<Option<String>>()
            .expect("there should be a solution")
    }
}

fn main() {
    solve::<Day06>(include_str!("../../inputs/day06.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = include_str!("../../inputs/day06-sample.txt");

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day06>(SAMPLE), "easter");
        assert_eq!(solve_part2::<Day06>(SAMPLE), "advent");
    }
}
