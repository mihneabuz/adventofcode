use lib::aoc;
use lib::challenge::Challenge;

use std::collections::HashSet;

pub struct Day3;

impl Challenge for Day3 {
    aoc!(year = 2022, day = 3);

    fn solve(input: String) -> (String, String) {
        let rucks = input.lines().collect::<Vec<&str>>();

        let res1 = rucks
            .iter()
            .fold(0, |acc, line| acc + priority(error(line)));

        let res2 = rucks
            .chunks(3)
            .into_iter()
            .fold(0, |acc, lines| acc + priority(common(lines)));

        (res1.to_string(), res2.to_string())
    }
}

fn error(s: &str) -> u8 {
    let (fst, snd) = s.split_at(s.len() / 2);

    let mut seen = vec![false; 128];
    for b in fst.bytes() {
        seen[b as usize] = true;
    }

    for b in snd.bytes() {
        if seen[b as usize] {
            return b;
        }
    }

    unreachable!()
}

fn common(ss: &[&str]) -> u8 {
    let c = ss
        .into_iter()
        .map(|s| s.bytes().collect::<HashSet<u8>>())
        .reduce(|set, next| set.intersection(&next).copied().collect::<HashSet<u8>>())
        .unwrap();

    c.into_iter().next().unwrap()
}

fn priority(b: u8) -> i32 {
    match b as char {
        'a'..='z' => (b - 'a' as u8) as i32 + 1,
        'A'..='Z' => (b - 'A' as u8) as i32 + 27,
        _ => unreachable!(),
    }
}
