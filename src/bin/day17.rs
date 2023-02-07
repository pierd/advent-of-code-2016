use aoc_helpers::{
    scaffold::{solve, Problem},
    walk,
};

struct Day17;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct State {
    x: isize,
    y: isize,
    path: String,
}

impl State {
    fn is_final(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

const MOVE_DIRECTION: [char; 4] = ['U', 'D', 'L', 'R'];
const MOVE_DELTA: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

struct StateTransitionsIterator {
    path: String,
    iter: std::vec::IntoIter<((isize, isize), char)>,
}

impl StateTransitionsIterator {
    fn new(prefix: &str, state: &State) -> Self {
        let digest: [u8; 16] = md5::compute(format!("{}{}", prefix, state.path)).into();
        let nibbles = [
            digest[0] >> 4,
            digest[0] & 0x0f,
            digest[1] >> 4,
            digest[1] & 0x0f,
        ];

        // prefill
        Self {
            path: state.path.clone(),
            iter: nibbles
                .into_iter()
                .zip(MOVE_DIRECTION.into_iter())
                .zip(MOVE_DELTA.into_iter())
                .filter_map(|((nibble, direction), (dx, dy))| {
                    let new_x = state.x + dx;
                    let new_y = state.y + dy;
                    if nibble >= 0x0b && (0..4).contains(&new_x) && (0..4).contains(&new_y) {
                        Some(((new_x, new_y), direction))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

impl Iterator for StateTransitionsIterator {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|((x, y), dir)| {
            let mut path = String::with_capacity(self.path.len() + 1);
            path.push_str(&self.path);
            path.push(dir);
            State { x, y, path }
        })
    }
}

struct FindFirstWalker {
    prefix: String,
}

impl FindFirstWalker {
    fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

impl walk::Walker<State> for FindFirstWalker {
    type NextGenerator = StateTransitionsIterator;
    type Result = String;

    fn visit(&mut self, state: &State) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if state.is_final() {
            return walk::VisitDecision::Break(state.path.clone());
        }
        walk::VisitDecision::Next(StateTransitionsIterator::new(&self.prefix, state))
    }
}

struct FindLongestWalker {
    prefix: String,
    best_length: usize,
}

impl FindLongestWalker {
    fn new(prefix: String) -> Self {
        Self {
            prefix,
            best_length: 0,
        }
    }
}

impl walk::Walker<State> for FindLongestWalker {
    type NextGenerator = StateTransitionsIterator;
    type Result = ();

    fn visit(&mut self, state: &State) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if state.is_final() {
            if self.best_length < state.path.len() {
                self.best_length = state.path.len()
            }
            return walk::VisitDecision::Continue;
        }
        walk::VisitDecision::Next(StateTransitionsIterator::new(&self.prefix, state))
    }
}

impl Problem for Day17 {
    type Input = String;
    type Part1 = String;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut walker = FindFirstWalker::new(input.to_owned());
        walk::walk_broad(&mut walker, Default::default())
            .expect("walker should break and there should be a solution")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut walker = FindLongestWalker::new(input.to_owned());
        walk::walk_deep(&mut walker, Default::default());
        walker.best_length
    }
}

fn main() {
    solve::<Day17>(include_str!("../../inputs/day17.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_sample_part1() {
        assert_eq!(solve_part1::<Day17>("ihgpwlah"), "DDRRRD");
        assert_eq!(solve_part1::<Day17>("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(
            solve_part1::<Day17>("ulqzkmiv"),
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(solve_part2::<Day17>("ihgpwlah"), 370);
        assert_eq!(solve_part2::<Day17>("kglvqrro"), 492);
        assert_eq!(solve_part2::<Day17>("ulqzkmiv"), 830);
    }
}
