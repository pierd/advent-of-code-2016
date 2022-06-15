use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};
use rematch::rematch;

struct Day20;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[rematch(r"(\d+)-(\d+)")]
struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn to_bounds(self) -> [Bound; 2] {
        [
            Bound {
                edge: self.lower,
                is_end: false,
            },
            Bound {
                edge: self.upper + 1,
                is_end: true,
            },
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bound {
    edge: usize,
    is_end: bool,
}

impl Problem for Day20 {
    type Input = VecFromLines<Range>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut bounds = input
            .iter()
            .flat_map(|r| r.to_bounds().into_iter())
            .collect::<Vec<_>>();
        bounds.sort_unstable();

        let mut active_ranges = 0;
        for Bound { edge, is_end } in bounds.into_iter() {
            active_ranges += if is_end { -1 } else { 1 };
            if active_ranges == 0 {
                return edge;
            }
        }
        unreachable!()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut bounds = input
            .iter()
            .flat_map(|r| r.to_bounds().into_iter())
            .collect::<Vec<_>>();
        bounds.sort_unstable();

        let mut active_ranges = 0;
        let mut count = 0;
        let mut open_start = None;
        for Bound { edge, is_end } in bounds.into_iter() {
            if is_end {
                active_ranges -= 1;
                if active_ranges == 0 {
                    open_start = Some(edge);
                }
            } else {
                if let Some(start) = open_start {
                    count += edge - start;
                }
                open_start = None;
                active_ranges += 1;
            }
        }
        count
    }
}

fn main() {
    solve::<Day20>(include_str!("../../inputs/day20.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    #[test]
    fn test_bound_sanity() {
        assert!(
            Bound {
                edge: 42,
                is_end: false
            } < Bound {
                edge: 42,
                is_end: true
            }
        );
    }

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day20>("5-8\n0-2\n4-7"), 3);
    }
}
