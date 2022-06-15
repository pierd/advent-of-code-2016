use std::{collections::HashMap, iter::repeat};

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
    walk,
};
use rematch::rematch;

struct Day22;

#[rematch(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T +\d+%")]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    available: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SimpleNode {
    Empty,
    Full,
    Stuck,
}

impl From<&Node> for SimpleNode {
    fn from(node: &Node) -> Self {
        if node.used == 0 {
            Self::Empty
        } else if node.used * 100 > node.size * 90 {
            Self::Stuck
        } else {
            Self::Full
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    grid: Vec<Vec<SimpleNode>>,
    wanted: (usize, usize),
    empty: (usize, usize),
}

impl State {
    fn is_final(&self) -> bool {
        self.wanted == (0, 0)
    }

    fn new_with_move(&self, (dx, dy): (isize, isize)) -> Option<Self> {
        let (target_x, target_y) = self.empty;
        assert_eq!(self.grid[target_y][target_x], SimpleNode::Empty);
        let x = usize::try_from(target_x as isize - dx);
        let y = usize::try_from(target_y as isize - dy);
        if let (Ok(x), Ok(y)) = (x, y) {
            if y < self.grid.len() && x < self.grid[y].len() && self.grid[y][x] != SimpleNode::Stuck
            {
                let mut new_state = self.clone();
                new_state.grid[target_y][target_x] = SimpleNode::Full;
                new_state.grid[y][x] = SimpleNode::Empty;
                if new_state.wanted == (x, y) {
                    new_state.wanted = (target_x, target_y);
                }
                new_state.empty = (x, y);
                return Some(new_state);
            }
        }
        None
    }
}

impl From<&[Node]> for State {
    fn from(nodes: &[Node]) -> Self {
        let max_x = nodes
            .iter()
            .map(|n| n.x)
            .max()
            .expect("there should be nodes provided");
        let max_y = nodes
            .iter()
            .map(|n| n.y)
            .max()
            .expect("there should be nodes provided");
        let mut grid = vec![vec![SimpleNode::Full; max_x + 1]; max_y + 1];
        for node in nodes {
            grid[node.y][node.x] = node.into();
        }
        let (empty_y, (empty_x, _)) = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| repeat(y).zip(row.iter().enumerate()))
            .find(|(_, (_, n))| **n == SimpleNode::Empty)
            .expect("there should be an empty node");
        Self {
            grid,
            wanted: (max_x, 0),
            empty: (empty_x, empty_y),
        }
    }
}

const MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Mover {
    move_cost: usize,
    starting_state: State,
}

impl walk::Generator<(usize, State)> for Mover {
    fn generate<F: FnMut((usize, State))>(&mut self, mut callback: F) {
        for mv in MOVES {
            if let Some(new_state) = self.starting_state.new_with_move(mv) {
                callback((self.move_cost, new_state));
            }
        }
    }
}

#[derive(Debug, Default)]
struct Walker {
    visited: HashMap<State, usize>,
}

impl walk::Walker<(usize, State)> for Walker {
    type NextGenerator = Mover;

    type Result = usize;

    fn visit(
        &mut self,
        (cost, state): &(usize, State),
    ) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if state.is_final() {
            walk::VisitDecision::Break(*cost)
        } else if self.visited.get(state).map(|c| cost < c).unwrap_or(true) {
            self.visited.insert(state.clone(), *cost);
            walk::VisitDecision::Next(Mover {
                move_cost: cost + 1,
                starting_state: state.clone(),
            })
        } else {
            // already visited with a lower cost
            walk::VisitDecision::Continue
        }
    }
}

impl Problem for Day22 {
    // Note: the input has 2 first lines chopped by hand
    type Input = VecFromLines<Node>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut count = 0;
        for first in input.iter() {
            if first.used == 0 {
                continue;
            }
            for second in input.iter() {
                if (first.x != second.x || first.y != second.y) && first.used <= second.available {
                    count += 1;
                }
            }
        }
        count
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        walk::walk_broad(&mut Walker::default(), (0, State::from(input.as_slice())))
            .expect("there should be a solution")
    }
}

fn main() {
    solve::<Day22>(include_str!("../../inputs/day22.txt"));
}
