use itertools::Itertools;
use lib::{aoc, challenge::Challenge, helpers::Bitset};

pub struct Day16;

impl Challenge for Day16 {
    aoc!(year = 2023, day = 16);

    fn solve(input: String) -> (String, String) {
        let map = input.lines().map(|line| line.as_bytes()).collect_vec();
        let n = map.len();

        let energized = |start: Point| {
            let mut light: Vec<Bitset<u8>> = vec![Bitset::new(); n * n];

            let mut points = vec![start];
            while !points.is_empty() {
                points = points
                    .drain(..)
                    .flat_map(|p| advance(p, &map, &mut light))
                    .filter_map(|p| p)
                    .collect();
            }

            light.iter().filter(|bit| !bit.is_empty()).count()
        };

        let fst = energized((0, 0, Dir::Right));

        let top = (0..n).map(|y| (0, y as isize, Dir::Down));
        let bot = (0..n).map(|y| (n as isize - 1, y as isize, Dir::Up));
        let lft = (0..n).map(|x| (x as isize, 0, Dir::Right));
        let rgt = (0..n).map(|x| (x as isize, n as isize - 1, Dir::Left));

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

type Point = (isize, isize, Dir);

fn advance(point: Point, map: &[&[u8]], light: &mut [Bitset<u8>]) -> [Option<Point>; 2] {
    let (x, y, dir) = point;

    if x < 0 || x as usize >= map.len() || y < 0 || y as usize >= map[0].len() {
        return [None, None];
    }

    let l = light
        .get_mut(x as usize * map[0].len() + y as usize)
        .unwrap();
    if l.get(dir as u8) {
        return [None, None];
    } else {
        *l = l.set(dir as u8);
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
