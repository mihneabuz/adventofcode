use hashbrown::{HashMap, HashSet};
use itertools::{iterate, Itertools};
use lib::{helpers::unchecked_parse, prelude::*};

pub struct Day22;

impl Challenge for Day22 {
    aoc!(year = 2024, day = 22);

    fn solve(input: String) -> (String, String) {
        let numbers = input.lines().map(unchecked_parse::<i64>).collect_vec();

        fn mix(secret: &mut i64, number: i64) {
            *secret ^= number;
        }

        fn prune(secret: &mut i64) {
            *secret %= 16777216;
        }

        fn evolve(secret: &i64) -> i64 {
            let mut next = *secret;

            let x = next * 64;
            mix(&mut next, x);
            prune(&mut next);

            let y = next / 32;
            mix(&mut next, y);
            prune(&mut next);

            let z = next * 2048;
            mix(&mut next, z);
            prune(&mut next);

            next
        }

        let res1 = numbers
            .iter()
            .map(|number| iterate(*number, evolve).nth(2000).unwrap())
            .sum::<i64>();

        let res2 = numbers
            .into_iter()
            .fold((HashMap::new(), 0), |(mut cache, mut best), number| {
                let secret = evolve(&number);
                let diff = (secret % 10 - number % 10) as i8;

                let mut seen = HashSet::new();
                iterate((secret, diff), |(secret, _)| {
                    let next = evolve(secret);
                    let diff = (next % 10 - secret % 10) as i8;

                    (next, diff)
                })
                .take(2000)
                .tuple_windows::<(_, _, _, _)>()
                .for_each(|window| {
                    let seq = [window.0 .1, window.1 .1, window.2 .1, window.3 .1];
                    let price = window.3 .0 % 10;
                    if !seen.contains(&seq) {
                        let total = *cache
                            .entry(seq)
                            .and_modify(|t| {
                                *t += price;
                            })
                            .or_insert(price);

                        best = best.max(total);

                        seen.insert(seq);
                    }
                });

                (cache, best)
            })
            .1;

        (res1.to_string(), res2.to_string())
    }
}
