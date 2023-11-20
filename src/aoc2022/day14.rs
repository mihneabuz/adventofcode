use lib::aoc;
use lib::challenge::Challenge;

use std::cmp;

const EMPTY: char = ' ';
const ROCK: char = '█';
const SAND: char = 'O';

type Point = (usize, usize);
type Line = Vec<Point>;
type Map = Vec<Vec<char>>;

pub struct Day14;

impl Challenge for Day14 {
    aoc!(year = 2022, day = 14);

    fn solve(input: String) -> (String, String) {
        let mut lines = input.lines().map(parse_lines).collect::<Vec<Line>>();

        let max_bounds = lines
            .iter()
            .flatten()
            .copied()
            .reduce(|(accx, accy), (x, y)| (cmp::max(accx, x), cmp::max(accy, y)))
            .unwrap();

        let min_bounds = lines
            .iter()
            .flatten()
            .copied()
            .reduce(|(accx, accy), (x, y)| (cmp::min(accx, x), cmp::min(accy, y)))
            .unwrap();

        let min_y = 0;
        let max_y = max_bounds.1 + 2;
        let min_x = min_bounds.0 - max_y;
        let max_x = max_bounds.0 + max_y;

        let sand_point = (500 - min_x, 0);
        lines
            .iter_mut()
            .flatten()
            .for_each(|p| *p = (p.0 - min_x, p.1 - min_y));

        let mut map = vec![vec![EMPTY; max_x - min_x]; max_y - min_y];

        lines.iter().for_each(|line| {
            line.iter().reduce(|prev, curr| {
                if prev.0 == curr.0 {
                    for i in cmp::min(prev.1, curr.1)..=cmp::max(prev.1, curr.1) {
                        map[i][curr.0] = ROCK;
                    }
                }

                if prev.1 == curr.1 {
                    for j in cmp::min(prev.0, curr.0)..=cmp::max(prev.0, curr.0) {
                        map[curr.1][j] = ROCK;
                    }
                }

                curr
            });
        });

        let (mut res1, mut res2) = (0, 0);
        for iter in 1.. {
            if !drop_sand(&mut map, sand_point) && res1 == 0 {
                res1 = iter - 1;
            }

            if map[sand_point.1][sand_point.0] == SAND && res2 == 0 {
                res2 = iter;
            }

            if res1 != 0 && res2 != 0 {
                break;
            }
        }

        (res1.to_string(), res2.to_string())
    }
}

fn parse_lines(s: &str) -> Line {
    s.split(" -> ")
        .map(|s| {
            let point = s.split_once(",").unwrap();
            (point.0.parse().unwrap(), point.1.parse().unwrap())
        })
        .collect()
}

fn drop_sand(map: &mut Map, drop_point: Point) -> bool {
    let (mut x, mut y) = drop_point;

    while y < map.len() - 1 {
        if map[y + 1][x] == EMPTY {
            y += 1;
        } else if map[y + 1][x - 1] == EMPTY {
            y += 1;
            x -= 1;
        } else if map[y + 1][x + 1] == EMPTY {
            y += 1;
            x += 1;
        } else {
            map[y][x] = SAND;
            return true;
        }
    }

    map[y][x] = SAND;
    false
}
