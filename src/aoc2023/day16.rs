use std::mem;

use itertools::Itertools;

use lib::{aoc, challenge::Challenge, helpers::Bitset};

pub struct Day16;

impl Challenge for Day16 {
    aoc!(year = 2023, day = 16);

    fn solve(input: String) -> (String, String) {
        let map = input.lines().map(|line| line.as_bytes()).collect_vec();
        let n = map.len();

        let energized = move |start: Point| {
            let mut light: Vec<Bitset<u8>> = vec![Bitset::new(); n * n];

            let (mut points, mut next) = (vec![start], vec![]);
            while !points.is_empty() {
                next.extend(
                    points
                        .drain(..)
                        .flat_map(|p| advance(p, &map, &mut light))
                        .flatten(),
                );

                mem::swap(&mut points, &mut next);
            }

            light.into_iter().filter(|set| !set.is_empty()).count()
        };

        let fst = energized((0, 0, Dir::Right));

        let top = (0..n).map(|y| (0, y as i32, Dir::Down));
        let bot = (0..n).map(|y| (n as i32 - 1, y as i32, Dir::Up));
        let lft = (0..n).map(|x| (x as i32, 0, Dir::Right));
        let rgt = (0..n).map(|x| (x as i32, n as i32 - 1, Dir::Left));

        let snd = top
            .chain(bot)
            .chain(lft)
            .chain(rgt)
            .map(energized)
            .max()
            .unwrap();

        (fst.to_string(), snd.to_string())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Dir {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

type Point = (i32, i32, Dir);

fn advance(point: Point, map: &[&[u8]], light: &mut [Bitset<u8>]) -> [Option<Point>; 2] {
    let (x, y, dir) = point;

    if x < 0 || x as usize >= map.len() || y < 0 || y as usize >= map[0].len() {
        return [None, None];
    }

    let light = &mut light[x as usize * map[0].len() + y as usize];
    if light.get(dir as u8) {
        return [None, None];
    } else {
        *light = light.set(dir as u8);
    }

    match (map[x as usize][y as usize], dir) {
        (b'\\', Dir::Up) => [Some((x, y - 1, Dir::Left)), None],
        (b'\\', Dir::Down) => [Some((x, y + 1, Dir::Right)), None],
        (b'\\', Dir::Left) => [Some((x - 1, y, Dir::Up)), None],
        (b'\\', Dir::Right) => [Some((x + 1, y, Dir::Down)), None],

        (b'/', Dir::Up) => [Some((x, y + 1, Dir::Right)), None],
        (b'/', Dir::Down) => [Some((x, y - 1, Dir::Left)), None],
        (b'/', Dir::Left) => [Some((x + 1, y, Dir::Down)), None],
        (b'/', Dir::Right) => [Some((x - 1, y, Dir::Up)), None],

        (b'|' | b'.', Dir::Up) => [Some((x - 1, y, Dir::Up)), None],
        (b'|' | b'.', Dir::Down) => [Some((x + 1, y, Dir::Down)), None],
        (b'|', Dir::Left | Dir::Right) => [Some((x - 1, y, Dir::Up)), Some((x + 1, y, Dir::Down))],

        (b'-' | b'.', Dir::Left) => [Some((x, y - 1, Dir::Left)), None],
        (b'-' | b'.', Dir::Right) => [Some((x, y + 1, Dir::Right)), None],
        (b'-', Dir::Up | Dir::Down) => [Some((x, y - 1, Dir::Left)), Some((x, y + 1, Dir::Right))],

        _ => unreachable!(),
    }
}
