use lib::{helpers::unchecked_parse, prelude::*};

use hashbrown::HashMap;
use itertools::Itertools;
use num::Integer;

fn blink(stone: u64, count: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if count == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(stone, count)) {
        return *res;
    }

    let res = if stone == 0 {
        blink(1, count - 1, cache)
    } else {
        let digits = stone.ilog10() + 1;
        if digits.is_even() {
            let pow = 10u64.pow(digits / 2);
            let upper = blink(stone / pow, count - 1, cache);
            let lower = blink(stone % pow, count - 1, cache);
            upper + lower
        } else {
            blink(stone * 2024, count - 1, cache)
        }
    };

    cache.insert((stone, count), res);

    res
}

pub struct Day11;

impl Challenge for Day11 {
    aoc!(year = 2024, day = 11);

    fn solve(input: String) -> (String, String) {
        let stones = input
            .split_whitespace()
            .map(unchecked_parse::<u64>)
            .collect_vec();

        let mut cache = HashMap::new();
        let mut solve = move |blinks| {
            stones
                .iter()
                .copied()
                .map(|stone| blink(stone, blinks, &mut cache))
                .sum::<u64>()
        };

        (solve(25).to_string(), solve(75).to_string())
    }
}
