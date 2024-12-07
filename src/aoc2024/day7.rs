use itertools::Itertools;
use lib::{helpers::unchecked_parse, prelude::*};

pub struct Day7;

fn can_solve<F, I>(values: &[u64], acc: u64, result: u64, op: F) -> bool
where
    F: Copy + Fn(u64, u64) -> I,
    I: Iterator<Item = u64>,
{
    if values.is_empty() {
        return acc == result;
    }

    if acc > result {
        return false;
    }

    let (head, rest) = values.split_first().unwrap();
    op(acc, *head).any(|acc| can_solve(rest, acc, result, op))
}

impl Challenge for Day7 {
    aoc!(year = 2024, day = 7);

    fn solve(input: String) -> (String, String) {
        let eqs = input
            .lines()
            .map(|line| {
                let (res, values) = line.split_once(": ").unwrap();
                let res = unchecked_parse::<u64>(res);
                let values = values
                    .split_whitespace()
                    .map(unchecked_parse::<u64>)
                    .collect_vec();
                (values, res)
            })
            .collect_vec();

        let pows: [u64; 20] = std::array::from_fn(|i| 10u64.pow(i as u32));
        let concat = |acc: u64, head: u64| {
            let digits = head.ilog10() + 1;
            acc * pows[digits as usize] + head
        };

        let (res1, res2) = eqs
            .iter()
            .fold((0, 0), |(mut acc1, mut acc2), (values, result)| {
                let s1 = can_solve(&values[1..], values[0], *result, |acc, head| {
                    [acc + head, acc * head].into_iter()
                });

                let s2 = can_solve(&values[1..], values[0], *result, |acc, head| {
                    [acc + head, acc * head, concat(acc, head)].into_iter()
                });

                if s1 {
                    acc1 += result;
                }

                if s2 {
                    acc2 += result;
                }

                (acc1, acc2)
            });

        (res1.to_string(), res2.to_string())
    }
}
