use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, RowsOfChars},
};

const KEYPAD: &[&[Option<char>]] = &[
    &[Some('1'), Some('2'), Some('3')],
    &[Some('4'), Some('5'), Some('6')],
    &[Some('7'), Some('8'), Some('9')],
];

const KEYPAD2: &[&[Option<char>]] = &[
    &[None, None, Some('1'), None, None],
    &[None, Some('2'), Some('3'), Some('4'), None],
    &[Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    &[None, Some('A'), Some('B'), Some('C'), None],
    &[None, None, Some('D'), None, None],
];

struct Keypad {
    keypad: &'static [&'static [Option<char>]],
    row: usize,
    col: usize,
}

impl Keypad {
    fn new(keypad: &'static [&'static [Option<char>]], row: usize, col: usize) -> Self {
        Self { keypad, row, col }
    }

    fn punch_in(&mut self, directions: &[Direction]) -> char {
        for direction in directions {
            let candidate = match direction {
                Direction::Up => (if self.row > 0 { self.row - 1 } else { self.row }, self.col),
                Direction::Down => (
                    if self.row < self.keypad.len() - 1 {
                        self.row + 1
                    } else {
                        self.row
                    },
                    self.col,
                ),
                Direction::Left => (self.row, if self.col > 0 { self.col - 1 } else { self.col }),
                Direction::Right => (
                    self.row,
                    if self.col < self.keypad[0].len() - 1 {
                        self.col + 1
                    } else {
                        self.col
                    },
                ),
            };
            if self.keypad[candidate.0][candidate.1].is_some() {
                self.row = candidate.0;
                self.col = candidate.1;
            }
        }
        self.keypad[self.row][self.col].unwrap()
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self {
            keypad: KEYPAD,
            row: 1,
            col: 1,
        }
    }
}

struct Day02;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow::anyhow!("Failed to parse: {}", value)),
        }
    }
}

impl Problem for Day02 {
    type Input = RowsOfChars<Direction>;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut keypad = Keypad::default();
        let mut result = String::new();
        for instr in input {
            result.push(keypad.punch_in(instr));
        }
        result
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut keypad = Keypad::new(KEYPAD2, 2, 0);
        let mut result = String::new();
        for instr in input {
            result.push(keypad.punch_in(instr));
        }
        result
    }
}

fn main() {
    solve::<Day02>(include_str!("../../inputs/day02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day02>(SAMPLE), "1985");
        assert_eq!(solve_part2::<Day02>(SAMPLE), "5DB3");
    }
}
