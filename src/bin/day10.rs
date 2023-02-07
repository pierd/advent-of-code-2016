use std::{collections::HashMap, str::FromStr};

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

struct Day10;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Command {
    Assign { num: usize, bot: usize },
    Bot { bot: usize, rule: BotRule },
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref ASSIGN_RE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
            static ref BOT_RE: Regex = Regex::new(
                r"bot (\d+) gives low to ((output|bot) \d+) and high to ((output|bot) \d+)"
            )
            .unwrap();
        }
        let parse_int = |caps: &Captures, group: usize| {
            caps.get(group)
                .ok_or_else(|| anyhow::anyhow!("Getting group {} failed: {}", group, s))?
                .as_str()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Int parse error: {}", e))
        };
        let parse_target = |caps: &Captures, group: usize| {
            caps.get(group)
                .ok_or_else(|| anyhow::anyhow!("Getting group {} failed: {}", group, s))?
                .as_str()
                .parse::<Target>()
                .map_err(|e| anyhow::anyhow!("Target parse error: {}", e))
        };
        if let Some(ref caps) = ASSIGN_RE.captures(s) {
            let num = parse_int(caps, 1)?;
            let bot = parse_int(caps, 2)?;
            Ok(Self::Assign { num, bot })
        } else if let Some(ref caps) = BOT_RE.captures(s) {
            let bot = parse_int(caps, 1)?;
            let low_target = parse_target(caps, 2)?;
            let high_target = parse_target(caps, 4)?;
            Ok(Self::Bot {
                bot,
                rule: BotRule {
                    low_target,
                    high_target,
                },
            })
        } else {
            Err(anyhow::anyhow!("Can't parse: {}", s))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Target {
    Bot(usize),
    Output(usize),
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().take(3).collect();
        match parts[..] {
            ["output", x] => Ok(Self::Output(
                x.parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Int parse error: {}", e))?,
            )),
            ["bot", x] => Ok(Self::Bot(
                x.parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Int parse error: {}", e))?,
            )),
            _ => Err(anyhow::anyhow!("Target parse error: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BotRule {
    low_target: Target,
    high_target: Target,
}

#[derive(Copy, Clone, Debug)]
enum BotVal {
    Empty,
    Partial(usize),
    Full(usize, usize),
}

impl BotVal {
    fn insert(&mut self, val: usize) -> Option<(usize, usize)> {
        match self {
            BotVal::Empty => {
                *self = Self::Partial(val);
                None
            }
            BotVal::Partial(x) => {
                *self = if *x < val {
                    Self::Full(*x, val)
                } else {
                    Self::Full(val, *x)
                };
                self.get_full()
            }
            _ => panic!("Can't insert to full bot!"),
        }
    }

    fn get_full(&self) -> Option<(usize, usize)> {
        match self {
            Self::Full(low, high) => Some((*low, *high)),
            _ => None,
        }
    }
}

impl Default for BotVal {
    fn default() -> Self {
        Self::Empty
    }
}

struct BotSystem {
    bot_rules: HashMap<usize, BotRule>,
    bot_vals: HashMap<usize, BotVal>,
    outputs: HashMap<usize, usize>,
}

impl BotSystem {
    fn from_commands(commands: &[Command]) -> Self {
        let mut bot_rules: HashMap<usize, BotRule> = HashMap::new();
        let mut bot_vals: HashMap<usize, BotVal> = HashMap::new();
        for command in commands {
            match *command {
                Command::Assign { num, bot } => {
                    bot_vals.entry(bot).or_default().insert(num);
                }
                Command::Bot { bot, rule } => {
                    let previous = bot_rules.insert(bot, rule);
                    assert!(previous.is_none());
                }
            }
        }
        Self {
            bot_rules,
            bot_vals,
            outputs: Default::default(),
        }
    }

    fn run(&mut self) {
        let mut full_bots: Vec<(usize, (usize, usize))> = self
            .bot_vals
            .iter()
            .filter_map(|(k, v)| v.get_full().map(|full| (*k, full)))
            .collect();
        while let Some((full_bot, (low, high))) = full_bots.pop() {
            if let Some(&BotRule {
                low_target,
                high_target,
            }) = self.bot_rules.get(&full_bot)
            {
                for (target, val) in [(low_target, low), (high_target, high)] {
                    match target {
                        Target::Bot(target) => {
                            if let Some(full_too) =
                                self.bot_vals.entry(target).or_default().insert(val)
                            {
                                full_bots.push((target, full_too));
                            }
                        }
                        Target::Output(target) => {
                            let existing = self.outputs.insert(target, val);
                            assert!(existing.is_none());
                        }
                    }
                }
            } else {
                panic!("No rule for bot: {full_bot}");
            }
        }
        assert!(full_bots.is_empty());
    }
}

impl Problem for Day10 {
    type Input = VecFromLines<Command>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut system = BotSystem::from_commands(input);
        system.run();
        system
            .bot_vals
            .into_iter()
            .find_map(|(bot, val)| {
                if let BotVal::Full(17, 61) = val {
                    Some(bot)
                } else {
                    None
                }
            })
            .expect("bot should be found")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut system = BotSystem::from_commands(input);
        system.run();
        let outputs = system.outputs;
        outputs[&0] * outputs[&1] * outputs[&2]
    }
}

fn main() {
    solve::<Day10>(include_str!("../../inputs/day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "bot 195 gives low to bot 4 and high to bot 130"
                .parse::<Command>()
                .unwrap(),
            Command::Bot {
                bot: 195,
                rule: BotRule {
                    low_target: Target::Bot(4),
                    high_target: Target::Bot(130)
                }
            }
        );
        assert_eq!(
            "bot 195 gives low to output 4 and high to bot 130"
                .parse::<Command>()
                .unwrap(),
            Command::Bot {
                bot: 195,
                rule: BotRule {
                    low_target: Target::Output(4),
                    high_target: Target::Bot(130)
                }
            }
        );
    }
}
