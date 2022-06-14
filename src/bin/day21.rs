use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};
use rematch::rematch;

struct Day21;

#[derive(Copy, Clone, Debug)]
#[rematch]
enum Instruction {
    #[rematch(r"swap position (\d+) with position (\d+)")]
    // means that the letters at indexes X and Y (counting from 0) should be swapped.
    SwapPositions(usize, usize),
    #[rematch(r"swap letter (.) with letter (.)")]
    // means that the letters X and Y should be swapped (regardless of where they appear in the string).
    SwapLetters(char, char),
    #[rematch(r"rotate left (\d+) steps?")]
    // means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
    RotateLeft(usize),
    #[rematch(r"rotate right (\d+) steps?")]
    // means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
    RotateRight(usize),
    #[rematch(r"rotate based on position of letter (.)")]
    // means that the whole string should be rotated to the right based on the index of letter X (counting from 0) as determined before this instruction does any rotations. Once the index is determined, rotate the string to the right one time, plus a number of times equal to that index, plus one additional time if the index was at least 4.
    RotatePosition(char),
    UnRotatePosition(char),
    #[rematch(r"reverse positions (\d+) through (\d+)")]
    // means that the span of letters at indexes X through Y (including the letters at X and Y) should be reversed in order.
    Reverse(usize, usize),
    #[rematch(r"move position (\d+) to position (\d+)")]
    // means that the letter which is at index X should be removed from the string, then inserted such that it ends up at index Y.
    Move(usize, usize),
}

impl Instruction {
    fn apply(&self, password: &mut [char]) {
        match *self {
            Instruction::SwapPositions(x, y) => password.swap(x, y),
            Instruction::SwapLetters(c1, c2) => {
                let idx1 = password
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == c1)
                    .expect("char should be in the password")
                    .0;
                let idx2 = password
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == c2)
                    .expect("char should be in the password")
                    .0;
                password.swap(idx1, idx2);
            }
            Instruction::RotateLeft(steps) => {
                password.rotate_left(steps % password.len());
            }
            Instruction::RotateRight(steps) => {
                password.rotate_right(steps % password.len());
            }
            Instruction::RotatePosition(c) => {
                let idx = password
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == c)
                    .expect("char should be in the password")
                    .0;
                let steps = idx + 1 + if idx >= 4 { 1 } else { 0 };
                password.rotate_right(steps % password.len());
            }
            Instruction::UnRotatePosition(c) => {
                let current_idx = password
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == c)
                    .expect("char should be in the password")
                    .0;

                // find the index we rotated from
                let mut idx = 0;
                while idx <= password.len() {
                    let steps = idx + 1 + if idx >= 4 { 1 } else { 0 };
                    if (idx + steps) % password.len() == current_idx {
                        break;
                    }
                    idx += 1;
                }
                assert!(idx < password.len(), "index not found");

                let steps = idx + 1 + if idx >= 4 { 1 } else { 0 };
                password.rotate_left(steps % password.len());
            }
            Instruction::Reverse(x, y) => {
                assert!(x <= y);
                password[x..=y].reverse();
            }
            #[allow(clippy::needless_collect)]
            Instruction::Move(x, y) => {
                let temp: Vec<char> = (0..password.len())
                    .into_iter()
                    .map(|idx| {
                        if (idx < x && idx < y) || (x < idx && y < idx) {
                            password[idx]
                        } else if idx == y {
                            password[x]
                        } else if x <= idx && idx < y {
                            password[idx + 1]
                        } else if y < idx && idx <= x {
                            password[idx - 1]
                        } else {
                            unreachable!()
                        }
                    })
                    .collect();
                for (p, t) in password.iter_mut().zip(temp.into_iter()) {
                    *p = t;
                }
            }
        }
    }

    fn reverse(&self) -> Self {
        match *self {
            Instruction::RotateLeft(steps) => Self::RotateRight(steps),
            Instruction::RotateRight(steps) => Self::RotateLeft(steps),
            Instruction::RotatePosition(c) => Self::UnRotatePosition(c),
            Instruction::UnRotatePosition(c) => Self::RotatePosition(c),
            Instruction::Move(x, y) => Self::Move(y, x),
            _ => *self,
        }
    }
}

impl Problem for Day21 {
    type Input = VecFromLines<Instruction>;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut password: Vec<char> = "abcdefgh".chars().collect();
        for instr in input {
            instr.apply(&mut password);
        }
        password.into_iter().collect()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut scrambbled_password: Vec<char> = "fbgdceah".chars().collect();
        for instr in input.iter().rev() {
            instr.reverse().apply(&mut scrambbled_password);
        }
        scrambbled_password.into_iter().collect()
    }
}

fn main() {
    solve::<Day21>(include_str!("../../inputs/day21.txt"));
}
