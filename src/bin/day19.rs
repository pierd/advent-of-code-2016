use std::collections::VecDeque;

use aoc_helpers::scaffold::{solve, Problem, TrimAndParse};

struct Day19;

impl Problem for Day19 {
    type Input = TrimAndParse<usize>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut queue = VecDeque::with_capacity(*input);
        queue.extend(1..=*input);
        while queue.len() > 1 {
            let taker = queue.pop_front().expect("there should be at least 2");
            queue.pop_front().expect("there should be at least 2");
            queue.push_back(taker);
        }
        queue.pop_front().expect("there should be exactly one")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut next_elf: Vec<usize> = (1..=*input + 1).into_iter().collect();
        *next_elf.first_mut().unwrap() = 0;
        *next_elf.last_mut().unwrap() = 1;
        let mut taker_idx = 1;
        let mut elfs = *input;

        // advance loser
        let mut pre_loser_idx = 1;
        let mut distance = elfs / 2;
        for _ in 0..(distance - 1) {
            pre_loser_idx = next_elf[pre_loser_idx];
        }

        loop {
            // remove loser (distance stays, count drops)
            let old_loser_idx = next_elf[pre_loser_idx];
            next_elf[pre_loser_idx] = next_elf[next_elf[pre_loser_idx]];
            next_elf[old_loser_idx] = 0; // mark removed for debugging
            elfs -= 1;
            if elfs == 1 {
                break;
            }

            // advance taker (distance drops, count stays)
            taker_idx = next_elf[taker_idx];
            distance -= 1;

            // check if we have to increase the distance
            assert!(distance <= elfs / 2);
            while distance < elfs / 2 {
                pre_loser_idx = next_elf[pre_loser_idx];
                distance += 1;
            }
        }
        taker_idx
    }
}

fn main() {
    solve::<Day19>(include_str!("../../inputs/day19.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day19>("5"), 3);
        assert_eq!(solve_part2::<Day19>("5"), 2);
    }
}
