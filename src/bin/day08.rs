use std::str::FromStr;

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};

struct Day08;

#[derive(Clone, Copy, Debug)]
enum Command {
    On { cols: usize, rows: usize },
    RotateRow { row: usize, by: usize },
    RotateColumn { col: usize, by: usize },
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // taking 6 to catch additional data at the end
        let parts = s.split_ascii_whitespace().take(6).collect::<Vec<_>>();
        match *parts.as_slice() {
            ["rect", dims_str] => {
                let mut dims = dims_str.split('x').map(|s| s.parse::<usize>());
                let mut get_dim = move || {
                    dims.next()
                        .ok_or_else(|| anyhow::anyhow!("Missing dimensions"))
                        .map_err(|e| anyhow::anyhow!("Dimensions parsing error: {}", e))
                };
                let cols = get_dim()??;
                let rows = get_dim()??;
                Ok(Self::On { cols, rows })
            }
            ["rotate", "row", y_eq_str, "by", by_str] => {
                let row = y_eq_str
                    .strip_prefix("y=")
                    .ok_or_else(|| anyhow::anyhow!("Expected 'y=' but found: {}", y_eq_str))?
                    .parse::<usize>()?;
                let by = by_str.parse::<usize>()?;
                Ok(Self::RotateRow { row, by })
            }
            ["rotate", "column", x_eq_str, "by", by_str] => {
                let col = x_eq_str
                    .strip_prefix("x=")
                    .ok_or_else(|| anyhow::anyhow!("Expected 'x=' but found: {}", x_eq_str))?
                    .parse::<usize>()?;
                let by = by_str.parse::<usize>()?;
                Ok(Self::RotateColumn { col, by })
            }
            _ => Err(anyhow::anyhow!("Can't parse: {:?}", s)),
        }
    }
}

#[derive(Clone, Debug)]
struct RotDisplay(Vec<Vec<bool>>);

impl RotDisplay {
    fn new(rows: usize, cols: usize) -> Self {
        Self(vec![vec![false; cols]; rows])
    }

    fn execute_all(&mut self, cmds: &[Command]) {
        for cmd in cmds {
            self.execute(cmd)
        }
    }

    fn execute(&mut self, cmd: &Command) {
        match *cmd {
            Command::On { cols, rows } => {
                for row in &mut self.0[0..rows] {
                    for p in &mut row[0..cols] {
                        *p = true;
                    }
                }
            }
            Command::RotateRow { row, by } => {
                let row_len = self.0[row].len();
                self.0[row].rotate_right(by % row_len);
            }
            Command::RotateColumn { col, by } => {
                let col_len = self.0.len();
                let mut column = self.0.iter().map(|row| row[col]).collect::<Vec<bool>>();
                column.rotate_right(by % col_len);
                for (p, rotated) in self
                    .0
                    .iter_mut()
                    .map(|row| &mut row[col])
                    .zip(column.into_iter())
                {
                    *p = rotated;
                }
            }
        }
    }

    fn count_pixels(&self) -> usize {
        self.0.iter().flat_map(|r| r.iter()).filter(|b| **b).count()
    }

    fn render(&self) -> String {
        let mut result = String::new();
        for row in self.0.iter().map(|row| {
            row.iter()
                .map(|p| if *p { '#' } else { '.' })
                .collect::<String>()
        }) {
            result.push('\n');
            result.push_str(&row);
        }
        result
    }
}

impl Problem for Day08 {
    type Input = VecFromLines<Command>;
    type Part1 = usize;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut display = RotDisplay::new(6, 50);
        display.execute_all(input);
        display.count_pixels()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut display = RotDisplay::new(6, 50);
        display.execute_all(input);
        display.render()
    }
}

fn main() {
    solve::<Day08>(include_str!("../../inputs/day08.txt"));
}
