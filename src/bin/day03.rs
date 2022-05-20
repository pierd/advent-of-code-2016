use std::str::FromStr;

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};

struct Triangle {
    sides: [usize; 3],
}

impl FromStr for Triangle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_ascii_whitespace().map(|p| p.parse::<usize>());
        let mut get_num = move || {
            nums.next()
                .ok_or_else(|| anyhow::anyhow!("not enough numbers"))
        };
        let sides = [get_num()??, get_num()??, get_num()??];
        Ok(Self { sides })
    }
}

impl Triangle {
    fn is_possible(&self) -> bool {
        self.sides[0] < self.sides[1] + self.sides[2]
            && self.sides[1] < self.sides[0] + self.sides[2]
            && self.sides[2] < self.sides[0] + self.sides[1]
    }
}

struct Day03;

fn repack_for_part2(mut input: &[Triangle]) -> Vec<Triangle> {
    assert!(input.len() % 3 == 0);
    let mut result = Vec::new();
    while !input.is_empty() {
        for i in 0..3 {
            result.push(Triangle {
                sides: [input[0].sides[i], input[1].sides[i], input[2].sides[i]],
            });
        }
        input = input.split_at(3).1;
    }
    result
}

impl Problem for Day03 {
    type Input = VecFromLines<Triangle>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().filter(|t| t.is_possible()).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let repacked = repack_for_part2(input);
        Self::solve_part1(&repacked)
    }
}

fn main() {
    solve::<Day03>(include_str!("../../inputs/day03.txt"));
}
