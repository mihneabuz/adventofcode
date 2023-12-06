use itertools::Itertools;
use num::integer::Roots;

use lib::{aoc, challenge::Challenge, helpers::unchecked_parse};

pub struct Day6;

impl Challenge for Day6 {
    aoc!(year = 2023, day = 6);

    fn solve(input: String) -> (String, String) {
        let (times, dists) = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(unchecked_parse::<isize>)
                    .collect_vec()
            })
            .collect_tuple()
            .unwrap();

        let fst = (0..times.len())
            .map(|i| solve(times[i], dists[i]))
            .product::<isize>();

        let (time, dist) = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .fold(String::new(), |acc, s| acc + s)
                    .parse::<isize>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        let snd = solve(time, dist);

        (fst.to_string(), snd.to_string())
    }
}

fn solve(time: isize, dist: isize) -> isize {
    let (mut lo, mut hi) = roots(-1, time, -dist).unwrap();

    while lo * (time - lo) <= dist {
        lo += 1;
    }

    while hi * (time - hi) <= dist {
        hi -= 1;
    }

    hi - lo + 1
}

fn roots(a: isize, b: isize, c: isize) -> Option<(isize, isize)> {
    let delta = b * b - 4 * a * c;
    if delta < 0 {
        return None;
    }

    let x1 = (-b - delta.sqrt()) / (2 * a);
    let x2 = (-b + delta.sqrt()) / (2 * a);

    let (lo, hi) = (x1.min(x2), x1.max(x2));

    Some((lo, hi))
}
