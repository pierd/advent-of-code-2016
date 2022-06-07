use std::str::FromStr;

use aoc_helpers::{
    anyhow, interpret,
    scaffold::{solve, Problem, VecFromLines},
};

#[derive(Copy, Clone, Debug, Default)]
struct State {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
}

impl State {
    fn get(&self, r: Register) -> isize {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    fn get_mut(&mut self, r: Register) -> &mut isize {
        match r {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }

    fn new_with_change<F: Fn(isize) -> isize>(&self, r: Register, change: F) -> Self {
        let mut new = *self;
        *new.get_mut(r) = change(new.get(r));
        new
    }

    fn eval(&self, oper: Operand) -> isize {
        match oper {
            Operand::Literal(x) => x,
            Operand::Register(r) => self.get(r),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(anyhow::anyhow!("Can't parse register: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Literal(isize),
    Register(Register),
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<isize>() {
            Ok(literal) => Ok(Self::Literal(literal)),
            Err(_) => Ok(Self::Register(s.parse::<Register>()?)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instr {
    Copy { from: Operand, to: Register },
    Incr(Register),
    Decr(Register),
    JumpNotZero { cond: Operand, offset: isize },
}

impl interpret::Instruction<State> for Instr {
    fn execute(&self, state: State) -> (State, interpret::Jump) {
        match *self {
            Instr::Copy { from, to } => {
                let val = state.eval(from);
                (state.new_with_change(to, |_| val), Default::default())
            }
            Instr::Incr(reg) => (state.new_with_change(reg, |x| x + 1), Default::default()),
            Instr::Decr(reg) => (state.new_with_change(reg, |x| x - 1), Default::default()),
            Instr::JumpNotZero { cond, offset } => {
                let val = state.eval(cond);
                (
                    state,
                    if val != 0 {
                        interpret::Jump::Relative(offset)
                    } else {
                        Default::default()
                    },
                )
            }
        }
    }
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().take(4).collect::<Vec<_>>();
        match parts[..] {
            ["Copy", operand, register] => Ok(Self::Copy {
                from: operand.parse::<Operand>()?,
                to: register.parse::<Register>()?,
            }),
            ["Incr", register] => Ok(Self::Incr(register.parse::<Register>()?)),
            ["Decr", register] => Ok(Self::Decr(register.parse::<Register>()?)),
            ["JumpNotZero", operand, int] => Ok(Self::JumpNotZero {
                cond: operand.parse::<Operand>()?,
                offset: int
                    .parse::<isize>()
                    .map_err(|e| anyhow::anyhow!("Integer parse error: {}", e))?,
            }),
            _ => Err(anyhow::anyhow!("Can't parse: {}", s)),
        }
    }
}

struct Day12;

impl Problem for Day12 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        interpret::execute_all_default(input).a
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        interpret::execute_all(
            State {
                a: 0,
                b: 0,
                c: 1,
                d: 0,
            },
            input,
        )
        .a
    }
}

fn main() {
    solve::<Day12>(include_str!("../../inputs/day12.txt"));
}
