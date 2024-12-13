use lib::prelude::*;

pub struct Day13;

impl Challenge for Day13 {
    aoc!(year = 2024, day = 13);

    fn solve(input: String) -> (String, String) {
        let re = regex::Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        let solve = |(ax, ay), (bx, by), (px, py)| {
            let cb = (ax * py - ay * px) / (ax * by - ay * bx);
            let ca = (px - bx * cb) / ax;

            let checkx = ax * ca + bx * cb == px;
            let checky = ay * ca + by * cb == py;

            (checkx && checky).then_some(3 * ca + cb)
        };

        const P: i64 = 10000000000000;

        let (res1, res2) = input
            .split("\n\n")
            .map(|game| {
                let m = re.captures(game).unwrap();
                let get = |i| m.get(i).unwrap().as_str().parse::<i64>().unwrap();

                let a = (get(1), get(2));
                let b = (get(3), get(4));
                let p = (get(5), get(6));

                (solve(a, b, p), solve(a, b, (p.0 + P, p.1 + P)))
            })
            .fold((0, 0), |(acc1, acc2), (res1, res2)| {
                (acc1 + res1.unwrap_or(0), acc2 + res2.unwrap_or(0))
            });

        (res1.to_string(), res2.to_string())
    }
}
