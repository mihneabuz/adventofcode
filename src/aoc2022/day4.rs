use lib::aoc;
use lib::challenge::Challenge;

pub struct Day4;

impl Challenge for Day4 {
    aoc!(year = 2022, day = 4);

    fn solve(input: String) -> (String, String) {
        let (mut res1, mut res2) = (0, 0);
        for line in input.lines() {
            let (r1, r2) = line.split_once(",").unwrap();
            let ((lo1, hi1), (lo2, hi2)) = (range(r1), range(r2));

            if (lo1 <= lo2 && hi1 >= hi2) || (lo2 <= lo1 && hi2 >= hi1) {
                res1 += 1;
            }

            if overlap(lo1, hi1, lo2, hi2) {
                res2 += 1;
            }
        }

        (res1.to_string(), res2.to_string())
    }
}

fn range(s: &str) -> (i32, i32) {
    let (lo, hi) = s.split_once("-").unwrap();
    (lo.parse().unwrap(), hi.parse().unwrap())
}

fn between(x: i32, a: i32, b: i32) -> bool {
    x >= a && x <= b
}

fn overlap(lo1: i32, hi1: i32, lo2: i32, hi2: i32) -> bool {
    between(lo1, lo2, hi2)
        || between(hi1, lo2, hi2)
        || between(lo2, lo1, hi1)
        || between(hi2, lo1, hi1)
}
