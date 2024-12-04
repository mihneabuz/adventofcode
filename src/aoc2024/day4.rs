use itertools::Itertools;
use lib::prelude::*;
use smallvec::{smallvec, SmallVec};

pub struct Day4;

impl Challenge for Day4 {
    aoc!(year = 2024, day = 4);

    fn solve(input: String) -> (String, String) {
        let map = input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect_vec();

        let (n, m) = (map.len(), map[0].len());

        let res1 = (0..n)
            .flat_map(|i| (0..m).map(move |j| (i, j)))
            .filter(|&(i, j)| map[i][j] == b'X')
            .map(|(i, j)| {
                let mut dis: SmallVec<[_; 3]> = smallvec![0];
                let mut djs: SmallVec<[_; 3]> = smallvec![0];

                if i < n - 3 {
                    dis.push(1);
                }

                if i >= 3 {
                    dis.push(-1);
                }

                if j < m - 3 {
                    djs.push(1);
                }

                if j >= 3 {
                    djs.push(-1);
                }

                let mut count = 0;
                for di in dis.iter() {
                    for dj in djs.iter() {
                        let i1 = (i as i32 + di) as usize;
                        let j1 = (j as i32 + dj) as usize;

                        let i2 = (i1 as i32 + di) as usize;
                        let j2 = (j1 as i32 + dj) as usize;

                        let i3 = (i2 as i32 + di) as usize;
                        let j3 = (j2 as i32 + dj) as usize;

                        if &[map[i1][j1], map[i2][j2], map[i3][j3]] == b"MAS" {
                            count += 1;
                        }
                    }
                }

                count
            })
            .sum::<u32>();

        let res2 = (1..n - 1)
            .flat_map(|i| (1..m - 1).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                if map[i][j] != b'A' {
                    return false;
                }

                let mut count = 0;
                let mut check = |x, y| {
                    if x == b'M' && y == b'S' {
                        count += 1
                    }
                };

                check(map[i - 1][j - 1], map[i + 1][j + 1]);
                check(map[i + 1][j + 1], map[i - 1][j - 1]);
                check(map[i - 1][j + 1], map[i + 1][j - 1]);
                check(map[i + 1][j - 1], map[i - 1][j + 1]);

                count >= 2
            })
            .count();

        (res1.to_string(), res2.to_string())
    }
}
