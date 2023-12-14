use std::collections::HashMap;

use itertools::Itertools;
use lib::{aoc, challenge::Challenge};

pub struct Day14;

impl Challenge for Day14 {
    aoc!(year = 2023, day = 14);

    fn solve(input: String) -> (String, String) {
        let mut map = input
            .lines()
            .map(|line| line.as_bytes().to_owned())
            .collect_vec();

        let risk = |map: &[Vec<u8>]| {
            map.iter()
                .enumerate()
                .map(|(i, line)| (map.len() - i) * line.iter().filter(|b| **b == b'O').count())
                .sum::<usize>()
        };

        let hash = |map: &[Vec<u8>]| {
            map.iter()
                .enumerate()
                .flat_map(|(i, line)| {
                    line.iter()
                        .enumerate()
                        .map(move |(j, c)| (*c == b'O') as usize * ((i + 1) * map.len()) * (j + 1))
                })
                .sum::<usize>()
        };

        cycle_north(&mut map);

        let fst = risk(&map);

        cycle_west(&mut map);
        cycle_south(&mut map);
        cycle_east(&mut map);

        let mut hashes = HashMap::new();
        let (i, repeat) = (1..)
            .find_map(|i| {
                let h = hash(&map);

                if let Some(prev) = hashes.get(&h) {
                    Some((i, i - prev))
                } else {
                    cycle(&mut map);
                    hashes.insert(h, i);
                    None
                }
            })
            .unwrap();

        for _ in 0..(1_000_000_000 - i) % repeat {
            cycle(&mut map);
        }

        let snd = risk(&map);

        (fst.to_string(), snd.to_string())
    }
}

fn cycle(map: &mut [Vec<u8>]) {
    cycle_north(map);
    cycle_west(map);
    cycle_south(map);
    cycle_east(map);
}

fn cycle_north(map: &mut [Vec<u8>]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != b'O' {
                continue;
            }

            let mut k = i;
            while k > 0 && map[k - 1][j] == b'.' {
                k -= 1;
            }

            map[i][j] = b'.';
            map[k][j] = b'O';
        }
    }
}

fn cycle_west(map: &mut [Vec<u8>]) {
    for line in map.iter_mut() {
        for j in 0..line.len() {
            if line[j] != b'O' {
                continue;
            }

            let mut k = j;
            while k > 0 && line[k - 1] == b'.' {
                k -= 1;
            }

            line[j] = b'.';
            line[k] = b'O';
        }
    }
}

fn cycle_south(map: &mut [Vec<u8>]) {
    for i in (0..map.len()).rev() {
        for j in 0..map[i].len() {
            if map[i][j] != b'O' {
                continue;
            }

            let mut k = i;
            while k < map.len() - 1 && map[k + 1][j] == b'.' {
                k += 1;
            }

            map[i][j] = b'.';
            map[k][j] = b'O';
        }
    }
}

fn cycle_east(map: &mut [Vec<u8>]) {
    for line in map.iter_mut() {
        for j in (0..line.len()).rev() {
            if line[j] != b'O' {
                continue;
            }

            let mut k = j;
            while k < line.len() - 1 && line[k + 1] == b'.' {
                k += 1;
            }

            line[j] = b'.';
            line[k] = b'O';
        }
    }
}
