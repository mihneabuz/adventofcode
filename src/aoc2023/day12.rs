use itertools::Itertools;
use lib::{aoc, challenge::Challenge, helpers::unchecked_parse};
use ndarray::Array3;

pub struct Day12;

impl Challenge for Day12 {
    aoc!(year = 2023, day = 12);

    fn solve(input: String) -> (String, String) {
        let lines = input
            .lines()
            .map(|line| {
                let (springs, meta) = line.split_once(' ').unwrap();
                let meta = meta.split(',').map(unchecked_parse::<usize>).collect_vec();
                (springs.as_bytes().to_owned(), meta)
            })
            .collect_vec();

        let fst = lines
            .iter()
            .map(|(springs, meta)| solve(springs.as_slice(), meta))
            .sum::<usize>();

        let snd = lines
            .into_iter()
            .map(|(springs, meta)| {
                let len = springs.len() * 5 + 4;
                let springs = springs
                    .into_iter()
                    .chain(Some(b'?'))
                    .cycle()
                    .take(len)
                    .collect_vec();
                let meta = meta.repeat(5);

                solve(springs.as_slice(), &meta)
            })
            .sum::<usize>();

        (fst.to_string(), snd.to_string())
    }
}

fn solve(springs: &[u8], counts: &[usize]) -> usize {
    let max_count = *counts.iter().max().unwrap();
    let mut dp = Array3::zeros((springs.len() + 1, counts.len() + 1, max_count + 2));

    dp[(0, 0, 0)] = 1;
    for (i, ch) in springs.iter().copied().enumerate() {
        for (j, count) in counts.iter().copied().enumerate() {
            for k in 0..=count {
                let x = dp[(i, j, k)];
                if x == 0 {
                    continue;
                }

                if matches!(ch, b'.' | b'?') {
                    if k == 0 {
                        dp[(i + 1, j, 0)] += x;
                    }

                    if k == count {
                        dp[(i + 1, j + 1, 0)] += x;
                    }
                }

                if matches!(ch, b'#' | b'?') {
                    dp[(i + 1, j, k + 1)] += x;
                }
            }
        }

        if matches!(ch, b'.' | b'?') {
            dp[(i + 1, counts.len(), 0)] += dp[(i, counts.len(), 0)];
        }
    }

    dp[(springs.len(), counts.len(), 0)]
        + dp[(springs.len(), counts.len() - 1, counts[counts.len() - 1])]
}
