use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, RowsOfChars},
};

struct Day18;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Safe),
            '^' => Ok(Self::Trap),
            _ => Err(anyhow::anyhow!("Can't parse: {:?}", value)),
        }
    }
}

fn iter_rows<F: FnMut(&[Tile])>(mut row: Vec<Tile>, times: usize, mut callback: F) {
    callback(&row);
    let mut next = row.clone();
    for _ in 1..times {
        for (idx, t) in next.iter_mut().enumerate() {
            let prev = [
                if idx == 0 { Tile::Safe } else { row[idx - 1] },
                if idx == row.len() - 1 {
                    Tile::Safe
                } else {
                    row[idx + 1]
                },
            ];
            *t = match prev {
                [Tile::Trap, Tile::Safe] | [Tile::Safe, Tile::Trap] => Tile::Trap,
                _ => Tile::Safe,
            };
        }
        callback(&next);
        row.swap_with_slice(&mut next);
    }
}

fn count_safe_tiles(row: Vec<Tile>, times: usize) -> usize {
    let mut count = 0;
    iter_rows(row, times, |row| {
        count += row.iter().filter(|t| **t == Tile::Safe).count()
    });
    count
}

impl Problem for Day18 {
    type Input = RowsOfChars<Tile>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        count_safe_tiles(input[0].clone(), 40)
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        count_safe_tiles(input[0].clone(), 400_000)
    }
}

fn main() {
    solve::<Day18>(include_str!("../../inputs/day18.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let row = ".^^.^.^^^^"
            .chars()
            .map(|c| Tile::try_from(c))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(count_safe_tiles(row, 10), 38);
    }
}
