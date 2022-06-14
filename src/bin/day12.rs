use advent_of_code_2016::{Instr, State};
use aoc_helpers::{
    interpret::Execute,
    scaffold::{solve, Problem, VecFromLines},
};

struct Day12;

impl Problem for Day12 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.execute(Default::default()).0.a
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input
            .execute(State {
                a: 0,
                b: 0,
                c: 1,
                d: 0,
            })
            .0
            .a
    }
}

fn main() {
    solve::<Day12>(include_str!("../../inputs/day12.txt"));
}
