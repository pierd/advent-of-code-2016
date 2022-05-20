use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromCommaSeparated},
};
use std::{collections::HashSet, str::FromStr};

struct Day01;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn process_moves(instrs: &[Self]) -> (isize, isize) {
        let mut position = (0, 0);
        let mut facing = (1isize, 0);
        for Self {
            direction,
            distance,
        } in instrs
        {
            let distance = *distance as isize;
            facing = match direction {
                Direction::Left => (facing.1, -facing.0),
                Direction::Right => (-facing.1, facing.0),
            };
            position.0 += facing.0 * distance;
            position.1 += facing.1 * distance;
        }
        position
    }

    fn walk_moves<F, T>(instrs: &[Self], mut callback: F) -> Option<T>
    where
        F: FnMut((isize, isize)) -> Option<T>,
    {
        let mut position = (0, 0);
        let mut facing = (1isize, 0);
        for Self {
            direction,
            distance,
        } in instrs
        {
            facing = match direction {
                Direction::Left => (facing.1, -facing.0),
                Direction::Right => (-facing.1, facing.0),
            };
            for _ in 0..*distance {
                position.0 += facing.0;
                position.1 += facing.1;
                if let Some(x) = callback(position) {
                    return Some(x);
                }
            }
        }
        None
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, rest) = s.split_at(1);
        let distance = rest.parse::<usize>()?;
        let direction = match prefix {
            "L" => Direction::Left,
            "R" => Direction::Right,
            s => return Err(anyhow::anyhow!("Unknown prefix: {}", s)),
        };
        Ok(Self {
            direction,
            distance,
        })
    }
}

impl Problem for Day01 {
    type Input = VecFromCommaSeparated<Instruction>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let (x, y) = Instruction::process_moves(input);
        x.abs() + y.abs()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut visited = HashSet::new();
        let (x, y) = Instruction::walk_moves(input, |position| {
            if visited.insert(position) {
                None
            } else {
                Some(position)
            }
        })
        .expect("position should repeat");
        x.abs() + y.abs()
    }
}

fn main() {
    solve::<Day01>(include_str!("../../inputs/day01.txt"));
}
