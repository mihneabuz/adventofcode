use lib::aoc;
use lib::challenge::Challenge;

use std::cmp;

pub struct Day8;

impl Challenge for Day8 {
    aoc!(year = 2022, day = 8);

    fn solve(input: String) -> (String, String) {
        let map = input
            .lines()
            .map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let n = map.len();

        let mut visible = vec![vec![false; n]; n];
        for i in 0..n {
            compute_visibility_horizontal(&map, i, &mut visible);
            compute_visibility_vertical(&map, i, &mut visible);
        }

        let count = visible
            .into_iter()
            .map(|v| v.into_iter().filter(|b| *b).count())
            .sum::<usize>();

        let mut best_score = 0;
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                let mut score = 1;

                score *= compute_score((0..i + 1).rev().map(|k| map[k][j]));
                score *= compute_score((i..n).map(|k| map[k][j]));

                score *= compute_score((0..j + 1).rev().map(|k| map[i][k]));
                score *= compute_score((j..n).map(|k| map[i][k]));

                best_score = cmp::max(best_score, score);
            }
        }

        (count.to_string(), best_score.to_string())
    }
}

fn compute_visibility_horizontal(map: &[Vec<u8>], i: usize, visible: &mut [Vec<bool>]) {
    let (mut lo, mut hi) = (0, map.len() - 1);
    let (mut max_lo, mut max_hi) = (map[i][lo], map[i][hi]);

    visible[i][lo] = true;
    visible[i][hi] = true;

    while lo < hi {
        if map[i][lo] <= map[i][hi] {
            lo += 1;
            if map[i][lo] > max_lo {
                max_lo = map[i][lo];
                visible[i][lo] = true;
            }
        } else {
            hi -= 1;
            if map[i][hi] > max_hi {
                max_hi = map[i][hi];
                visible[i][hi] = true;
            }
        }
    }
}

fn compute_visibility_vertical(map: &[Vec<u8>], j: usize, visible: &mut [Vec<bool>]) {
    let (mut lo, mut hi) = (0, map.len() - 1);
    let (mut max_lo, mut max_hi) = (map[lo][j], map[hi][j]);

    visible[lo][j] = true;
    visible[hi][j] = true;

    while lo < hi {
        if map[lo][j] <= map[hi][j] {
            lo += 1;
            if map[lo][j] > max_lo {
                max_lo = map[lo][j];
                visible[lo][j] = true;
            }
        } else {
            hi -= 1;
            if map[hi][j] > max_hi {
                max_hi = map[hi][j];
                visible[hi][j] = true;
            }
        }
    }
}

fn compute_score(mut iter: impl Iterator<Item = u8>) -> i32 {
    let start = iter.next().unwrap();
    let mut score = 0;

    for next in iter {
        score += 1;

        if next >= start {
            break;
        }
    }

    score
}
