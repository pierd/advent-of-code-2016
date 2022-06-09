use aoc_helpers::{
    bfs,
    scaffold::{solve, Problem, TrimAndParse},
};

struct Day13;

#[derive(Clone, Copy, Debug)]
struct MazeDriver {
    magic: isize,
}

impl MazeDriver {
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn new(magic: isize) -> Self {
        Self { magic }
    }

    fn is_a_wall(&self, (x, y): (isize, isize)) -> bool {
        if x < 0 || y < 0 {
            true
        } else {
            (x * x + 3 * x + 2 * x * y + y + y * y + self.magic).count_ones() % 2 == 1
        }
    }
}

impl bfs::FlatCostDriver<(isize, isize)> for MazeDriver {
    type TransitionsIterator = Box<dyn Iterator<Item = (isize, isize)>>;

    fn iter_transitions(&self, (x, y): &(isize, isize)) -> Self::TransitionsIterator {
        let (x, y) = (*x, *y);
        let drv = *self;
        Box::new(
            Self::DIRECTIONS
                .iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(move |point| !drv.is_a_wall(*point)),
        )
    }

    fn is_final(&self, state: &(isize, isize)) -> bool {
        state == &(31, 39)
    }
}

impl Problem for Day13 {
    type Input = TrimAndParse<isize>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let driver = MazeDriver::new(*input);
        bfs::find_lowest_cost(&driver, 0, (1, 1), None)
            .final_cost
            .expect("there should be a solution")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let driver = MazeDriver::new(*input);
        let result = bfs::find_lowest_cost(&driver, 0, (1, 1), Some(53));
        result
            .seen_states
            .into_iter()
            .filter(|(_, cost)| *cost <= 50)
            .count()
    }
}

fn main() {
    solve::<Day13>(include_str!("../../inputs/day13.txt"));
}
