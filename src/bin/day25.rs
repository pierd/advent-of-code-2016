use std::collections::HashSet;

use advent_of_code_2016::{Instr, State};
use aoc_helpers::{
    interpret::{Execute, Jump},
    scaffold::{solve, Problem, VecFromLines},
};

struct Day25;

fn outputs_clock(
    instrs: &[Instr],
    mut state: State,
    seen_states: &mut HashSet<(usize, State)>,
) -> bool {
    let mut last_output = 1;
    let mut output_count = 0;
    let mut instr_idx = 0;
    while let Some(instr) = instrs.get(instr_idx) {
        let (new_state, jump) = if let &Instr::Output(oper) = instr {
            let output = state.eval(oper);
            if output != 1 - last_output {
                return false;
            }
            last_output = output;
            output_count += 1;
            (state, Default::default())
        } else {
            instr.execute(state)
        };
        state = new_state;
        match jump {
            Jump::Absolute(idx) => instr_idx = idx,
            Jump::Relative(d) => {
                if let Ok(new_idx) = usize::try_from(instr_idx as isize + d) {
                    instr_idx = new_idx
                } else {
                    break;
                }
            }
            Jump::Stop => break,
        }
        if seen_states.contains(&(instr_idx, new_state)) {
            break;
        }
        seen_states.insert((instr_idx, new_state));
    }
    output_count >= 2
}

impl Problem for Day25 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut seen_states = HashSet::new();
        (0..)
            .into_iter()
            .find(|a| {
                outputs_clock(
                    input,
                    State {
                        a: *a,
                        b: 0,
                        c: 0,
                        d: 0,
                    },
                    &mut seen_states,
                )
            })
            .expect("there should be a solution")
    }

    fn solve_part2(_input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        "N/A".to_owned()
    }
}

fn main() {
    solve::<Day25>(include_str!("../../inputs/day25.txt"));
}
