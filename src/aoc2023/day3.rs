use itertools::Itertools;
use lib::{aoc, challenge::Challenge, example};

pub struct Day3;

impl Challenge for Day3 {
    aoc!(year = 2023, day = 3);

    example!(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    );
    fn solve(input: String) -> (String, String) {
        let map = input.lines().map(|l| l.as_bytes()).collect_vec();
        let (n, m) = (map.len(), map[0].len());

        let (mut fst, mut snd) = (0, 0);
        for i in 0..n {
            for j in 0..m {
                if is_symbol(map[i][j]) {
                    let mut nums = Vec::new();
                    for (ni, nj) in get_neighs(i, j, n, m) {
                        if map[ni][nj].is_ascii_digit() {
                            nums.push((ni, expand(&map[ni], nj)));
                        }
                    }

                    nums.sort();
                    nums.dedup();

                    if map[i][j] == b'*' && nums.len() == 2 {
                        let mut prod = 1;
                        for (i, (l, r)) in nums.iter().copied() {
                            let s = String::from_utf8_lossy(&map[i][l..r + 1]);
                            prod *= s.parse::<usize>().unwrap();
                        }
                        snd += prod;
                    }

                    for (i, (l, r)) in nums {
                        let s = String::from_utf8_lossy(&map[i][l..r + 1]);
                        fst += s.parse::<usize>().unwrap();
                    }
                }
            }
        }

        (fst.to_string(), snd.to_string())
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn get_neighs(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let dirs = [-1, 0, 1];

    let mut neighs = Vec::with_capacity(8);
    for di in dirs {
        let x = i as isize + di;
        if x < 0 || x >= n as isize {
            continue;
        }

        for dj in dirs {
            let y = j as isize + dj;
            if y < 0 || y >= m as isize {
                continue;
            }

            neighs.push((x as usize, y as usize));
        }
    }

    neighs
}

fn expand(row: &[u8], j: usize) -> (usize, usize) {
    let (mut l, mut r) = (j, j);

    while l > 0 && row[l - 1].is_ascii_digit() {
        l -= 1;
    }

    while r < row.len() - 1 && row[r + 1].is_ascii_digit() {
        r += 1;
    }

    (l, r)
}
