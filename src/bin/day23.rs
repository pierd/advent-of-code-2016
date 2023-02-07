use advent_of_code_2016::{Instr, Operand, State};
use aoc_helpers::{
    interpret::{Execute, Jump},
    scaffold::{solve, Problem, VecFromLines},
};

struct Day23;

fn toggle(instr: Instr) -> Instr {
    match instr {
        Instr::Copy { from, to } => Instr::JumpNotZero {
            cond: from,
            offset: Operand::Register(to),
        },
        Instr::Incr(x) => Instr::Decr(x),
        Instr::Toggle(Operand::Register(x))
        | Instr::Output(Operand::Register(x))
        | Instr::Decr(x) => Instr::Incr(x),
        Instr::JumpNotZero {
            cond,
            offset: Operand::Register(x),
        } => Instr::Copy { from: cond, to: x },
        _ => panic!("can't toggle: {instr:?}"),
    }
}

fn eval_toggle_instr(instrs: &mut [Instr], toggle_instr_idx: usize, state: State) -> (State, Jump) {
    if let Some(&Instr::Toggle(oper)) = instrs.get(toggle_instr_idx) {
        let d = state.eval(oper);
        if let Ok(instr_idx) = usize::try_from(toggle_instr_idx as isize + d) {
            if let Some(instr) = instrs.get_mut(instr_idx) {
                *instr = toggle(*instr);
            }
        }
    } else {
        panic!("should be called with a toggle instruction data");
    }
    (state, Default::default())
}

fn execute_with_toggle(instrs: &mut [Instr], mut state: State) -> State {
    let mut instr_idx = 0;
    while let Some(instr) = instrs.get(instr_idx) {
        let (new_state, jump) = if let Instr::Toggle(_) = instr {
            eval_toggle_instr(instrs, instr_idx, state)
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
    }
    state
}

impl Problem for Day23 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        execute_with_toggle(
            &mut input.clone(),
            State {
                a: 7,
                b: 0,
                c: 0,
                d: 0,
            },
        )
        .a
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        execute_with_toggle(
            &mut input.clone(),
            State {
                a: 12,
                b: 0,
                c: 0,
                d: 0,
            },
        )
        .a
    }
}

fn main() {
    solve::<Day23>(include_str!("../../inputs/day23.txt"));
}
