use lib::{aoc, challenge::Challenge};

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2024, day = 1);

    fn solve(input: String) -> (String, String) {
        let (mut xs, mut ys): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once("   ").unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .unzip();

        xs.sort();
        ys.sort();

        let res1 = xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| x.abs_diff(*y))
            .sum::<u32>();

        let res2 = xs
            .into_iter()
            .map(|x| {
                if let Ok(mut i) = ys.binary_search(&x) {
                    let mut count = 0;
                    while ys[i] == x {
                        count += 1;
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }

                    x * count as u32
                } else {
                    0
                }
            })
            .sum::<u32>();

        (res1.to_string(), res2.to_string())
    }
}
