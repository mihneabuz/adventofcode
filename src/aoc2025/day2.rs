use lib::prelude::*;

pub struct Day2;

impl Challenge for Day2 {
    aoc!(year = 2025, day = 2);

    fn solve(input: String) -> (String, String) {
        let (mut fst, mut snd) = (0, 0);

        let ranges = input.split(',').map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        });

        for (start, end) in ranges {
            for x in start..=end {
                let digits = x.ilog10() + 1;

                if digits % 2 == 0 {
                    let mid = 10u64.pow(digits / 2);
                    if x / mid == x % mid {
                        fst += x;
                    }
                }

                for group in 1..=5 {
                    if digits % group == 0 && digits / group > 1 {
                        let pow = 10u64.pow(group);
                        let target = x % pow;
                        let mut copy = x / pow;
                        let mut good = true;

                        while copy > 0 {
                            if copy % pow != target {
                                good = false;
                                break;
                            }

                            copy = copy / pow;
                        }

                        if good {
                            snd += x;
                            break;
                        }
                    }
                }
            }
        }

        (fst.to_string(), snd.to_string())
    }
}
