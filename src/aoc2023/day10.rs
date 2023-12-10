use itertools::Itertools;
use lib::{aoc, challenge::Challenge, helpers::Map};

pub struct Day10;

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Challenge for Day10 {
    aoc!(year = 2023, day = 10);

    fn solve(input: String) -> (String, String) {
        let map = Map::from_slices(&input.lines().map(|line| line.as_bytes()).collect_vec());
        let mut main_pipe = vec![false; map.height() * map.width()];

        let start = map
            .positions()
            .find(|&(i, j)| *map.get(i, j) == b'S')
            .unwrap();

        let (mut pos, mut dir) = {
            if matches!(*map.get(start.0 - 1, start.1), b'|' | b'7' | b'F') {
                ((start.0 - 1, start.1), Dir::Up)
            } else if matches!(*map.get(start.0 + 1, start.1), b'|' | b'L' | b'J') {
                ((start.0 + 1, start.1), Dir::Down)
            } else {
                ((start.0, start.1 + 1), Dir::Right)
            }
        };

        let fst = (1..)
            .find(|_| {
                main_pipe[pos.0 * map.width() + pos.1] = true;

                dir = match (*map.get(pos.0, pos.1), dir) {
                    (b'|', Dir::Up) => Dir::Up,
                    (b'|', Dir::Down) => Dir::Down,
                    (b'-', Dir::Left) => Dir::Left,
                    (b'-', Dir::Right) => Dir::Right,
                    (b'L', Dir::Down) => Dir::Right,
                    (b'L', Dir::Left) => Dir::Up,
                    (b'J', Dir::Down) => Dir::Left,
                    (b'J', Dir::Right) => Dir::Up,
                    (b'7', Dir::Up) => Dir::Left,
                    (b'7', Dir::Right) => Dir::Down,
                    (b'F', Dir::Up) => Dir::Right,
                    (b'F', Dir::Left) => Dir::Down,
                    (b'S', _) => return true,
                    _ => unreachable!(),
                };

                pos = match dir {
                    Dir::Up => (pos.0 - 1, pos.1),
                    Dir::Down => (pos.0 + 1, pos.1),
                    Dir::Left => (pos.0, pos.1 - 1),
                    Dir::Right => (pos.0, pos.1 + 1),
                };

                false
            })
            .unwrap()
            / 2;

        let mut inside = false;
        let snd = map
            .cells()
            .zip(main_pipe)
            .filter(|&((pos, cell), is_pipe)| {
                if pos.1 == 0 {
                    inside = false;
                }

                inside ^= is_pipe && matches!(*cell, b'|' | b'L' | b'J');
                inside && !is_pipe
            })
            .count();

        (fst.to_string(), snd.to_string())
    }
}
