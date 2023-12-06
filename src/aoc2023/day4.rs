use std::collections::VecDeque;

use lib::{
    aoc,
    challenge::Challenge,
    helpers::{unchecked_parse, Bitmap},
};

pub struct Day4;

impl Challenge for Day4 {
    aoc!(year = 2023, day = 4);

    fn solve(input: String) -> (String, String) {
        let (mut fst, mut snd) = (0, 0);
        let mut copies = VecDeque::new();

        input
            .lines()
            .map(|l| parse_line(l.split_once(": ").unwrap().1))
            .for_each(|(winning, guess)| {
                let count = winning.intersect(guess).count_bits() as usize;

                fst += if count > 0 {
                    2usize.pow(count as u32 - 1)
                } else {
                    0
                };

                let extra = copies.pop_front().unwrap_or(1);
                if copies.len() < count {
                    copies.resize(count, 1);
                }
                copies.range_mut(..count).for_each(|c| *c += extra);

                snd += extra;
            });

        (fst.to_string(), snd.to_string())
    }
}

fn parse_line(line: &str) -> (Bitmap<u128>, Bitmap<u128>) {
    let (winning_str, guess_str) = line.split_once(" | ").unwrap();

    let winning: Bitmap<u128> = winning_str
        .split_whitespace()
        .map(unchecked_parse::<usize>)
        .fold(Bitmap::new(), |acc, num| acc.set(num));

    let guess: Bitmap<u128> = guess_str
        .split_whitespace()
        .map(unchecked_parse::<usize>)
        .fold(Bitmap::new(), |acc, num| acc.set(num));

    (winning, guess)
}
