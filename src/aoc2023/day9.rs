use itertools::Itertools;
use lib::{aoc, challenge::Challenge, helpers::unchecked_parse};

pub struct Day9;

impl Challenge for Day9 {
    aoc!(year = 2023, day = 9);

    fn solve(input: String) -> (String, String) {
        let metrics = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(unchecked_parse::<isize>)
                    .collect_vec()
            })
            .collect_vec();

        let len = metrics.first().unwrap().len();
        let mut pyramid = (1..len - 1)
            .rev()
            .map(|l| Vec::with_capacity(l + 4))
            .collect_vec();

        let (fst, snd) = metrics
            .into_iter()
            .map(|metric| {
                let mut height = 0;
                pyramid[height].extend(metric.windows(2).map(|w| w[1] - w[0]));
                while !pyramid[height].iter().all(|e| *e == 0) {
                    height += 1;
                    let (prev, next) = pyramid.split_at_mut(height);
                    next[0].extend(prev.last().unwrap().windows(2).map(|w| w[1] - w[0]));
                }

                let interp_tail = metric.last().unwrap()
                    + pyramid[0..height]
                        .iter()
                        .rev()
                        .fold(0, |acc, row| row.last().unwrap() + acc);

                let interp_head = metric.first().unwrap()
                    - pyramid[0..height]
                        .iter()
                        .rev()
                        .fold(0, |acc, row| row.first().unwrap() - acc);

                pyramid[0..height + 1].iter_mut().for_each(|v| v.clear());

                (interp_tail, interp_head)
            })
            .reduce(|(accx, accy), (x, y)| (accx + x, accy + y))
            .unwrap();

        (fst.to_string(), snd.to_string())
    }
}
