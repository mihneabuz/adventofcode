use lib::aoc;
use lib::challenge::Challenge;

pub struct Day20;

impl Challenge for Day20 {
    aoc!(year = 2022, day = 20);

    fn solve(input: String) -> (String, String) {
        let values = input
            .lines()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let fst = solve(&values, 1, 1);
        let snd = solve(&values, 10, 811589153);

        (fst.to_string(), snd.to_string())
    }
}

fn next_index(i: i64, val: i64, n: i64) -> i64 {
    if i + val > 0 {
        (i + val) % n
    } else {
        n + (i + val) % n
    }
}

fn solve(values: &Vec<i64>, rounds: i32, key: i64) -> i64 {
    let n = values.len();

    let mut xs = values
        .iter()
        .copied()
        .map(|x| x * key)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..rounds {
        for k in 0..n {
            let (i, &v) = xs.iter().enumerate().find(|e| e.1 .0 == k).unwrap();

            let j = next_index(i as i64, v.1, (n - 1) as i64) as usize;
            if j > i {
                for k in i..j {
                    xs[k] = xs[k + 1];
                }
                xs[j] = v;
            } else {
                for k in (j..i).rev() {
                    xs[k + 1] = xs[k];
                }
                xs[j] = v;
            }
        }
    }

    let offset = xs.iter().enumerate().find(|(_, v)| v.1 == 0).unwrap().0;
    [1000, 2000, 3000]
        .iter()
        .map(|i| xs[(i + offset) % n].1)
        .sum::<i64>()
}
