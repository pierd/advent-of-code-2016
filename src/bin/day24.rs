use std::{collections::HashMap, iter::repeat};

use aoc_helpers::{
    anyhow, permutations,
    scaffold::{solve, Problem, RowsOfChars},
    walk,
};

struct Day24;

const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy, Debug)]
enum Field {
    Wall,
    Passage,
    Point(u8),
}

impl Field {
    fn is_passable(&self) -> bool {
        match self {
            Field::Wall => false,
            Field::Passage => true,
            Field::Point(_) => true,
        }
    }
}

impl TryFrom<char> for Field {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Passage),
            '0'..='9' => Ok(Self::Point(value as u8 - b'0')),
            _ => Err(anyhow::anyhow!("Can't parse {:?}", value)),
        }
    }
}

fn find_points(map: &[Vec<Field>]) -> HashMap<u8, (usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| repeat(row_idx).zip(row.iter().enumerate()))
        .filter_map(|(row_idx, (col_idx, f))| {
            if let &Field::Point(idx) = f {
                Some((idx, (row_idx, col_idx)))
            } else {
                None
            }
        })
        .collect()
}

struct MapStepsGenerator<'m> {
    map: &'m [Vec<Field>],
    step_cost: usize,
    from: (usize, usize),
}

impl<'m> walk::Generator<(usize, (usize, usize))> for MapStepsGenerator<'m> {
    fn generate<F: FnMut((usize, (usize, usize)))>(&mut self, mut callback: F) {
        for (drow, dcol) in MOVES {
            if let (Ok(to_row), Ok(to_col)) = (
                usize::try_from(self.from.0 as isize + drow),
                usize::try_from(self.from.1 as isize + dcol),
            ) {
                if to_row < self.map.len()
                    && to_col < self.map[to_row].len()
                    && self.map[to_row][to_col].is_passable()
                {
                    callback((self.step_cost, (to_row, to_col)));
                }
            }
        }
    }
}

struct DistanceFindingWalker<'m> {
    map: &'m [Vec<Field>],
    target: (usize, usize),
    visited: HashMap<(usize, usize), usize>,
}

impl<'m> walk::Walker<(usize, (usize, usize))> for DistanceFindingWalker<'m> {
    type NextGenerator = MapStepsGenerator<'m>;

    type Result = usize;

    fn visit(
        &mut self,
        (cost, state): &(usize, (usize, usize)),
    ) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if self.target == *state {
            walk::VisitDecision::Break(*cost)
        } else if self.visited.get(state).map(|c| cost < c).unwrap_or(true) {
            self.visited.insert(*state, *cost);
            walk::VisitDecision::Next(MapStepsGenerator {
                map: self.map,
                step_cost: cost + 1,
                from: *state,
            })
        } else {
            // already visited with a lower cost
            walk::VisitDecision::Continue
        }
    }
}

fn find_distance(map: &[Vec<Field>], from: (usize, usize), to: (usize, usize)) -> usize {
    walk::walk_broad(
        &mut DistanceFindingWalker {
            map,
            target: to,
            visited: Default::default(),
        },
        (0, from),
    )
    .expect("point should be reachable")
}

impl Problem for Day24 {
    type Input = RowsOfChars<Field>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(map: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let points = find_points(map);
        let mut distances: HashMap<(u8, u8), usize> = Default::default();
        for (pid1, p1) in points.iter() {
            for (pid2, p2) in points.iter() {
                let distance = find_distance(map, *p1, *p2);
                distances.insert((*pid1, *pid2), distance);
                distances.insert((*pid2, *pid1), distance);
            }
        }

        let pids_without_0: Vec<u8> = points
            .iter()
            .filter_map(|(pid, _)| if *pid != 0 { Some(*pid) } else { None })
            .collect();
        let mut best = usize::MAX;
        permutations::permutations(pids_without_0, |perm| {
            let mut cost = 0;
            let mut last = 0;
            for pid in perm {
                cost += distances
                    .get(&(last, *pid))
                    .expect("distance should have been calculated");
                last = *pid;
            }
            if cost < best {
                best = cost;
            }
        });
        best
    }

    fn solve_part2(map: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let points = find_points(map);
        let mut distances: HashMap<(u8, u8), usize> = Default::default();
        for (pid1, p1) in points.iter() {
            for (pid2, p2) in points.iter() {
                let distance = find_distance(map, *p1, *p2);
                distances.insert((*pid1, *pid2), distance);
                distances.insert((*pid2, *pid1), distance);
            }
        }

        let pids_without_0: Vec<u8> = points
            .iter()
            .filter_map(|(pid, _)| if *pid != 0 { Some(*pid) } else { None })
            .collect();
        let mut best = usize::MAX;
        permutations::permutations(pids_without_0, |perm| {
            let mut cost = 0;
            let mut last = 0;
            for pid in perm {
                cost += distances
                    .get(&(last, *pid))
                    .expect("distance should have been calculated");
                last = *pid;
            }
            cost += distances
                .get(&(last, 0))
                .expect("distance should have been calculated");
            if cost < best {
                best = cost;
            }
        });
        best
    }
}

fn main() {
    solve::<Day24>(include_str!("../../inputs/day24.txt"));
}
