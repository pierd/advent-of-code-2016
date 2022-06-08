use aoc_helpers::scaffold::{solve, Problem};
use regex::Regex;

struct Day09;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CharGroup {
    Normal(usize),
    Repeated {
        chars: usize,
        times: usize,
        raw_len: usize,
    },
}

#[derive(Clone, Copy, Debug)]
struct ConsumeResult {
    consumed: usize,
    left_over: usize,
}

impl CharGroup {
    fn len(&self) -> usize {
        match *self {
            CharGroup::Normal(l) => l,
            CharGroup::Repeated { raw_len, .. } => raw_len,
        }
    }

    fn consume_len(&self, len: usize) -> ConsumeResult {
        let self_len = self.len();
        if self_len < len {
            ConsumeResult {
                consumed: self_len,
                left_over: 0,
            }
        } else {
            ConsumeResult {
                consumed: len,
                left_over: self_len - len,
            }
        }
    }
}

struct CharGroupSeq {
    groups: Vec<CharGroup>,
}

impl CharGroupSeq {
    fn new(raw: &str) -> Self {
        lazy_static::lazy_static! {
            static ref GROUP_RE: Regex = Regex::new(r"[A-Z]+|\(\d+x\d+\)").unwrap();
            static ref REPEATED_RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
        };

        let groups = GROUP_RE
            .find_iter(raw)
            .map(|mtch| {
                if mtch.as_str().starts_with('(') {
                    let caps = REPEATED_RE
                        .captures(mtch.as_str())
                        .expect("already matched");
                    CharGroup::Repeated {
                        chars: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        times: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        raw_len: mtch.range().len(),
                    }
                } else {
                    CharGroup::Normal(mtch.range().len())
                }
            })
            .collect();

        Self { groups }
    }

    fn decompressed_len(&self) -> usize {
        let mut len = 0;
        let mut consuming = 0;
        for grp in &self.groups {
            if consuming > 0 {
                let ConsumeResult {
                    consumed,
                    left_over,
                } = grp.consume_len(consuming);
                consuming -= consumed;
                len += left_over;
            } else {
                match *grp {
                    CharGroup::Normal(l) => len += l,
                    CharGroup::Repeated { chars, times, .. } => {
                        len += chars * times;
                        consuming = chars;
                    }
                }
            }
        }
        len
    }

    fn denormalised(&self) -> Self {
        let mut groups = Vec::new();
        for group in &self.groups {
            match *group {
                CharGroup::Normal(count) => {
                    for _ in 0..count {
                        groups.push(CharGroup::Normal(1));
                    }
                }
                CharGroup::Repeated { .. } => groups.push(*group),
            }
        }
        Self { groups }
    }
}

// TODO: test
fn split_after_raw_len(
    groups: &[CharGroup],
    mut len: usize,
) -> Option<(&[CharGroup], &[CharGroup])> {
    for (idx, group) in groups.iter().enumerate() {
        if len < group.len() {
            return Some(groups.split_at(idx));
        }
        len -= group.len();
    }
    if len == 0 {
        Some((groups, &[]))
    } else {
        None
    }
}

fn is_a_tree(groups: &[CharGroup]) -> bool {
    if let Some((first, rest)) = groups.split_first() {
        match *first {
            CharGroup::Normal(_) => is_a_tree(rest),
            CharGroup::Repeated { chars, .. } => {
                if let Some((first_group, second_group)) = split_after_raw_len(rest, chars) {
                    is_a_tree(first_group) && is_a_tree(second_group)
                } else {
                    false
                }
            }
        }
    } else {
        true
    }
}

fn decompressed_tree_len(groups: &[CharGroup]) -> usize {
    if let Some((first, rest)) = groups.split_first() {
        match *first {
            CharGroup::Normal(l) => l + decompressed_tree_len(rest),
            CharGroup::Repeated { chars, times, .. } => {
                if let Some((first_group, second_group)) = split_after_raw_len(rest, chars) {
                    times * decompressed_tree_len(first_group) + decompressed_tree_len(second_group)
                } else {
                    panic!("Not a tree!");
                }
            }
        }
    } else {
        // empty
        0
    }
}

impl Problem for Day09 {
    type Input = String;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        CharGroupSeq::new(input).decompressed_len()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let char_groups = CharGroupSeq::new(input).denormalised();
        debug_assert!(is_a_tree(&char_groups.groups));
        decompressed_tree_len(&char_groups.groups)
    }
}

fn main() {
    solve::<Day09>(include_str!("../../inputs/day09.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_after_raw_len() {
        assert_eq!(
            split_after_raw_len(&[CharGroup::Normal(4),], 4),
            Some(([CharGroup::Normal(4)].as_slice(), [].as_slice()))
        );

        assert_eq!(
            split_after_raw_len(
                &[
                    CharGroup::Normal(4),
                    CharGroup::Repeated {
                        chars: 4,
                        times: 2,
                        raw_len: 5
                    },
                    CharGroup::Normal(4),
                ],
                4
            ),
            Some((
                [CharGroup::Normal(4)].as_slice(),
                [
                    CharGroup::Repeated {
                        chars: 4,
                        times: 2,
                        raw_len: 5
                    },
                    CharGroup::Normal(4),
                ]
                .as_slice()
            ))
        );
    }

    #[test]
    fn test_is_a_tree() {
        assert!(is_a_tree(&[CharGroup::Normal(4)]));
        assert!(is_a_tree(&[
            CharGroup::Repeated {
                chars: 4,
                times: 2,
                raw_len: 5
            },
            CharGroup::Normal(4)
        ]));
        assert!(is_a_tree(&[
            CharGroup::Normal(4),
            CharGroup::Repeated {
                chars: 4,
                times: 2,
                raw_len: 5
            },
            CharGroup::Normal(4),
        ]));
    }
}
