use std::cmp::Ordering;

use lib::{helpers::unchecked_parse, prelude::*};

use itertools::Itertools;

pub struct Day2;

fn is_safe(report: impl Iterator<Item = i32>) -> bool {
    let not_safe = report
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| (a.cmp(&b), a.abs_diff(b) <= 3))
        .scan((None, true), |acc, (a, b)| {
            if acc.0.is_none() {
                acc.0 = Some(a);
            }

            if a == Ordering::Equal || a != acc.0.unwrap() {
                return Some(false);
            }

            Some(acc.1 && b)
        })
        .find(|safe| !safe);

    not_safe.unwrap_or(true)
}

impl Challenge for Day2 {
    aoc!(year = 2024, day = 2);

    fn solve(input: String) -> (String, String) {
        let reports = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(unchecked_parse::<i32>)
                    .collect_vec()
            })
            .collect_vec();

        let res1 = reports
            .iter()
            .filter(|r| is_safe(r.iter().copied()))
            .count();

        let res2 = reports
            .iter()
            .filter(|report| {
                report
                    .iter()
                    .combinations(report.len() - 1)
                    .any(|report| is_safe(report.into_iter().copied()))
            })
            .count();

        (res1.to_string(), res2.to_string())
    }
}
