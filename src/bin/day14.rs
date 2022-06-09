use std::collections::{HashMap, VecDeque};

use aoc_helpers::scaffold::{solve, Problem};

struct Day14;

fn hash(rounds: usize, s: &str) -> String {
    let mut result = s.to_owned();
    for _ in 0..rounds {
        result = format!("{:x}", md5::compute(result));
    }
    result
}

fn produce_hashes(rounds: usize, prefix: String) -> impl Iterator<Item = (usize, String)> {
    (0..)
        .into_iter()
        .map(move |n| (n, hash(rounds, &format!("{}{}", prefix, n))))
}

fn find_3_and_5(s: &str) -> (Option<u8>, Option<u8>) {
    (
        s.as_bytes().windows(3).find_map(|w| {
            if w[0] == w[1] && w[1] == w[2] {
                Some(w[0])
            } else {
                None
            }
        }),
        s.as_bytes().windows(5).find_map(|w| {
            if w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4] {
                Some(w[0])
            } else {
                None
            }
        }),
    )
}

fn find_64th_n(hashing_rounds: usize, prefix: String) -> usize {
    let mut next = {
        let mut nums_iter =
            produce_hashes(hashing_rounds, prefix).map(|(n, s)| (n, find_3_and_5(&s)));
        move || nums_iter.next().expect("there should always be next hash")
    };

    let mut queue = VecDeque::new();
    let mut fives: HashMap<u8, usize> = HashMap::new();

    // add 1001 elements to the queue for processing
    for _ in 0..=1000 {
        let (n, (set3, set5)) = next();
        if let Some(x) = set5 {
            *fives.entry(x).or_default() += 1;
        }
        queue.push_back((n, set3, set5));
    }

    let mut nums_found = 0;
    let mut last_num_found = 0;
    while nums_found < 64 {
        // remove one
        let (n, set3, set5) = queue.pop_front().expect("shouldn't be empty");

        // discard all 5s provided by this number
        if let Some(x) = set5 {
            *fives.get_mut(&x).expect("should already be inserted") -= 1;
        }

        // check if any of the 3s from the removed num is in the remaining 5s (from the next 1000s hashes)
        if let Some(x) = set3 {
            if fives.get(&x).map(|v| *v > 0).unwrap_or_default() {
                nums_found += 1;
                last_num_found = n;
            }
        }

        // add next one
        let (n, (set3, set5)) = next();
        if let Some(x) = set5 {
            *fives.entry(x).or_default() += 1;
        }
        queue.push_back((n, set3, set5));
    }
    last_num_found
}

impl Problem for Day14 {
    type Input = String;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        find_64th_n(1, input.to_owned())
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        find_64th_n(2017, input.to_owned())
    }
}

fn main() {
    solve::<Day14>(include_str!("../../inputs/day14.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "abc";

    #[test]
    fn test_hashing() {
        assert_eq!(hash(1, "abc0"), "577571be4de9dcce85a041ba0410f29f");
        assert_eq!(hash(2, "abc0"), "eec80a0c92dc8a0777c619d9bb51e910");
        assert_eq!(hash(3, "abc0"), "16062ce768787384c81fe17a7a60c7e3");
        assert_eq!(hash(2017, "abc0"), "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn test_sample_part1() {
        assert_eq!(solve_part1::<Day14>(SAMPLE), 22728);
    }

    #[test]
    #[ignore = "slow"]
    fn test_sample_part2() {
        assert_eq!(solve_part2::<Day14>(SAMPLE), 22551);
    }
}
