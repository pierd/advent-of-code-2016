use aoc_helpers::{
    anyhow,
    interpret::{Execute, Jump},
};
use rematch::rematch;

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct State {
    pub a: isize,
    pub b: isize,
    pub c: isize,
    pub d: isize,
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

    pub fn eval(&self, oper: Operand) -> isize {
        match oper {
            Operand::Literal(x) => x,
            Operand::Register(r) => self.get(r),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[rematch]
pub enum Register {
    #[rematch(r"a")]
    A,
    #[rematch(r"b")]
    B,
    #[rematch(r"c")]
    C,
    #[rematch(r"d")]
    D,
}

#[derive(Copy, Clone, Debug)]
#[rematch]
pub enum Operand {
    #[rematch(r"(-?\d+)")]
    Literal(isize),
    #[rematch(r"(a|b|c|d)")]
    Register(Register),
}

#[derive(Copy, Clone, Debug)]
#[rematch]
pub enum Instr {
    #[rematch(r"cpy (-?\d+|a|b|c|d) (a|b|c|d)")]
    Copy { from: Operand, to: Register },
    #[rematch(r"inc (a|b|c|d)")]
    Incr(Register),
    #[rematch(r"dec (a|b|c|d)")]
    Decr(Register),
    #[rematch(r"jnz (-?\d+|a|b|c|d) (-?\d+|a|b|c|d)")]
    JumpNotZero { cond: Operand, offset: Operand },
    #[rematch(r"tgl (-?\d+|a|b|c|d)")]
    Toggle(Operand),
    #[rematch(r"out (-?\d+|a|b|c|d)")]
    Output(Operand),
}

impl Execute<State> for Instr {
    fn execute(&self, state: State) -> (State, Jump) {
        match *self {
            Instr::Copy { from, to } => {
                let val = state.eval(from);
                (state.new_with_change(to, |_| val), Default::default())
            }
            Instr::Incr(reg) => (state.new_with_change(reg, |x| x + 1), Default::default()),
            Instr::Decr(reg) => (state.new_with_change(reg, |x| x - 1), Default::default()),
            Instr::JumpNotZero { cond, offset } => {
                let val = state.eval(cond);
                let offset = state.eval(offset);
                (
                    state,
                    if val != 0 {
                        Jump::Relative(offset)
                    } else {
                        Default::default()
                    },
                )
            }
            Instr::Toggle(_) | Instr::Output(_) => {
                // no-op for this trait
                (state, Default::default())
            }
        }
    }
}
