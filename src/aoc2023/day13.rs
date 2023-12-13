use std::cmp;

use closure::closure;
use itertools::Itertools;

use lib::{aoc, challenge::Challenge};

pub struct Day13;

impl Challenge for Day13 {
    aoc!(year = 2023, day = 13);

    fn solve(input: String) -> (String, String) {
        let (fst, snd) = input.split("\n\n")
            .map(|m| {
                let grid = m.lines().map(|s| s.as_bytes()).collect_vec();
                let (n, m) = (grid.len(), grid[0].len());

                let (mut fst, mut snd) = (0, 0);

                for mid in 1..n {
                    let wrong = (0..cmp::min(mid, n - mid))
                        .flat_map(|i| {
                            (0..m).filter(closure!(move i, ref grid, |&j| grid[mid - 1 - i][j] != grid[mid + i][j]))
                        })
                        .take(2)
                        .count();

                    match wrong {
                        0 => fst += 100 * mid,
                        1 => snd += 100 * mid,
                        _ => {},
                    }
                }

                for mid in 1..m {
                    let wrong = (0..cmp::min(mid, m - mid))
                        .flat_map(|j| {
                            (0..n).filter(closure!(move j, ref grid, |&i| grid[i][mid - 1 - j] != grid[i][mid + j]))
                        })
                        .take(2)
                        .count();

                    match wrong {
                        0 => fst += mid,
                        1 => snd += mid,
                        _ => {},
                    }
                }

                (fst, snd)
            })
            .reduce(|acc, (fst, snd)| (acc.0 + fst, acc.1 + snd))
            .unwrap();

        (fst.to_string(), snd.to_string())
    }
}
