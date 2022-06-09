use std::str::FromStr;

use aoc_helpers::{
    anyhow, bfs,
    scaffold::{solve, Problem, VecFromLines},
};

const ELEMENTS: [&str; 7] = [
    "promethium",
    "cobalt",
    "curium",
    "ruthenium",
    "plutonium",
    "elerium",
    "dilithium",
];
const EXTENDED_COUNT: usize = 2; // last 2 elements are extended

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Floor {
    generators: [bool; ELEMENTS.len()],
    chips: [bool; ELEMENTS.len()],
}

impl Floor {
    fn is_safe(&self) -> bool {
        if !self.generators.iter().any(|b| *b) {
            // no generators -> safe
            return true;
        }
        for (chip, generator) in self.chips.iter().zip(self.generators.iter()) {
            if *chip && !*generator {
                // chip without its generator -> not safe
                return false;
            }
        }
        // all chips connected to their generators
        true
    }
}

impl FromStr for Floor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Self = Default::default();
        for (idx, element) in ELEMENTS.into_iter().enumerate() {
            result.generators[idx] = s.contains(&format!("a {} generator", element));
            result.chips[idx] = s.contains(&format!("a {}-compatible microchip", element));
        }
        Ok(result)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct BuildingState {
    floors: [Floor; 4],
    elevator: usize,
}

impl BuildingState {
    fn new(floors: &[Floor]) -> Self {
        let floors = [floors[0], floors[1], floors[2], floors[3]];
        Self {
            floors,
            elevator: 0,
        }
    }

    fn new_extended(floors: &[Floor]) -> Self {
        let mut result = Self::new(floors);
        for idx in (ELEMENTS.len() - EXTENDED_COUNT)..ELEMENTS.len() {
            result.floors[0].chips[idx] = true;
            result.floors[0].generators[idx] = true;
        }
        result
    }

    fn is_final(&self) -> bool {
        const SIMPLE_LEN: usize = ELEMENTS.len() - EXTENDED_COUNT;
        const EMPTY: [bool; SIMPLE_LEN] = [false; SIMPLE_LEN];
        const FULL: [bool; SIMPLE_LEN] = [true; SIMPLE_LEN];
        self.floors[0].chips[..SIMPLE_LEN] == EMPTY
            && self.floors[0].generators[..SIMPLE_LEN] == EMPTY
            && self.floors[1].chips[..SIMPLE_LEN] == EMPTY
            && self.floors[1].generators[..SIMPLE_LEN] == EMPTY
            && self.floors[2].chips[..SIMPLE_LEN] == EMPTY
            && self.floors[2].generators[..SIMPLE_LEN] == EMPTY
            && self.floors[3].chips[..SIMPLE_LEN] == FULL
            && self.floors[3].generators[..SIMPLE_LEN] == FULL
    }

    fn is_extended_final(&self) -> bool {
        const EMPTY: [bool; ELEMENTS.len()] = [false; ELEMENTS.len()];
        const FULL: [bool; ELEMENTS.len()] = [true; ELEMENTS.len()];
        self.floors[0].chips == EMPTY
            && self.floors[0].generators == EMPTY
            && self.floors[1].chips == EMPTY
            && self.floors[1].generators == EMPTY
            && self.floors[2].chips == EMPTY
            && self.floors[2].generators == EMPTY
            && self.floors[3].chips == FULL
            && self.floors[3].generators == FULL
    }

    fn try_move<F, G>(&self, target_floors: &[usize], update_fun: F, mut success_move_callback: G)
    where
        F: Fn(&mut Floor, bool),
        G: FnMut(Self),
    {
        let mut new_state = *self;
        update_fun(&mut new_state.floors[self.elevator], false);
        if new_state.floors[self.elevator].is_safe() {
            // taking the chip is safe
            for target_floor in target_floors {
                new_state.elevator = *target_floor;
                update_fun(&mut new_state.floors[*target_floor], true);
                if new_state.floors[*target_floor].is_safe() {
                    // bringing the chip to target_floor is safe too (so the move is safe)
                    success_move_callback(new_state);
                }
                // take the chip back for the next iteration
                update_fun(&mut new_state.floors[*target_floor], false);
            }
        }
    }

    fn possible_next_states(&self) -> Vec<Self> {
        let mut states = Vec::new();

        // possible target floors
        let target_floors = if self.elevator == 0 {
            vec![self.elevator + 1]
        } else if self.elevator == self.floors.len() - 1 {
            vec![self.elevator - 1]
        } else {
            vec![self.elevator + 1, self.elevator - 1]
        };

        // move 1 chip and 1 gen
        for (chip_idx, chip_present) in self.floors[self.elevator].chips.iter().enumerate() {
            if *chip_present {
                for (gen_idx, gen_present) in
                    self.floors[self.elevator].generators.iter().enumerate()
                {
                    if *gen_present {
                        self.try_move(
                            &target_floors,
                            |floor, val| {
                                floor.chips[chip_idx] = val;
                                floor.generators[gen_idx] = val;
                            },
                            |state| {
                                states.push(state);
                            },
                        );
                    }
                }
            }
        }

        // move 1 chip
        for (chip_idx, chip_present) in self.floors[self.elevator].chips.iter().enumerate() {
            if *chip_present {
                self.try_move(
                    &target_floors,
                    |floor, val| floor.chips[chip_idx] = val,
                    |state| {
                        states.push(state);
                    },
                );
            }
        }

        // move 2 chips
        for (chip_idx, chip_present) in self.floors[self.elevator].chips.iter().enumerate() {
            if *chip_present {
                for (chip2_idx, chip2_present) in self.floors[self.elevator]
                    .chips
                    .iter()
                    .enumerate()
                    .skip(chip_idx)
                {
                    if *chip2_present {
                        self.try_move(
                            &target_floors,
                            |floor, val| {
                                floor.chips[chip_idx] = val;
                                floor.chips[chip2_idx] = val;
                            },
                            |state| {
                                states.push(state);
                            },
                        );
                    }
                }
            }
        }

        // move 1 gen
        for (gen_idx, gen_present) in self.floors[self.elevator].generators.iter().enumerate() {
            if *gen_present {
                self.try_move(
                    &target_floors,
                    |floor, val| floor.generators[gen_idx] = val,
                    |state| {
                        states.push(state);
                    },
                );
            }
        }

        // move 2 gen
        for (gen_idx, gen_present) in self.floors[self.elevator].generators.iter().enumerate() {
            if *gen_present {
                for (gen2_idx, gen2_present) in self.floors[self.elevator]
                    .generators
                    .iter()
                    .enumerate()
                    .skip(gen_idx)
                {
                    if *gen2_present {
                        self.try_move(
                            &target_floors,
                            |floor, val| {
                                floor.generators[gen_idx] = val;
                                floor.generators[gen2_idx] = val;
                            },
                            |state| {
                                states.push(state);
                            },
                        );
                    }
                }
            }
        }

        states
    }
}

struct Driver {
    extended: bool,
}

impl bfs::FlatCostDriver<BuildingState> for Driver {
    type TransitionsIterator = std::vec::IntoIter<BuildingState>;

    fn iter_transitions(&self, from_state: &BuildingState) -> Self::TransitionsIterator {
        from_state.possible_next_states().into_iter()
    }

    fn is_final(&self, state: &BuildingState) -> bool {
        if self.extended {
            state.is_extended_final()
        } else {
            state.is_final()
        }
    }
}

struct Day11;

impl Problem for Day11 {
    type Input = VecFromLines<Floor>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let building = BuildingState::new(input);
        let result = bfs::find_lowest_cost(&Driver { extended: false }, 0, building, None);
        result.final_cost.expect("there should be a solution")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let building = BuildingState::new_extended(input);
        let result = bfs::find_lowest_cost(&Driver { extended: true }, 0, building, None);
        result.final_cost.expect("there should be a solution")
    }
}

fn main() {
    solve::<Day11>(include_str!("../../inputs/day11.txt"));
}
