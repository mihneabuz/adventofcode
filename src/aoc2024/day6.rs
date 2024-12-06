use lib::{helpers, prelude::*};

use bitflags::bitflags;
use itertools::{iterate, Itertools};
use ndarray::Array2;

pub struct Day6;

type Pos = (i32, i32);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Dir: u8 {
        const UP    = 0b0001;
        const DOWN  = 0b0010;
        const LEFT  = 0b0100;
        const RIGHT = 0b1000;
    }
}

impl Dir {
    pub fn di(&self) -> i32 {
        match *self {
            Dir::UP => -1,
            Dir::RIGHT => 0,
            Dir::DOWN => 1,
            Dir::LEFT => 0,
            _ => panic!(),
        }
    }

    pub fn dj(&self) -> i32 {
        match *self {
            Dir::UP => 0,
            Dir::RIGHT => 1,
            Dir::DOWN => 0,
            Dir::LEFT => -1,
            _ => panic!(),
        }
    }

    pub fn turn_right(&self) -> Self {
        match *self {
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,

            _ => panic!(),
        }
    }
}

fn step(pos: Pos, dir: Dir) -> Pos {
    ((pos.0 + dir.di()), (pos.1 + dir.dj()))
}

fn valid(pos: Pos, bounds: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 as i32 && pos.1 >= 0 && pos.1 < bounds.1 as i32
}

fn advance(pos: Pos, dir: Dir, map: &Array2<u8>) -> (Pos, Dir) {
    let next = step(pos, dir);
    if valid(next, map.dim()) && map[(next.0 as usize, next.1 as usize)] == b'#' {
        (pos, dir.turn_right())
    } else {
        (next, dir)
    }
}

impl Challenge for Day6 {
    aoc!(year = 2024, day = 6);

    fn solve(input: String) -> (String, String) {
        let mut map = helpers::array2::from_str(&input);
        let (n, m) = map.dim();

        let start = map
            .indexed_iter()
            .find_map(|((i, j), b)| (*b == b'^').then_some((i as i32, j as i32)))
            .unwrap();

        let path = iterate((start, Dir::UP), |&(pos, dir)| advance(pos, dir, &map))
            .take_while(|(pos, _)| valid(*pos, (n, m)))
            .collect_vec();

        let mut cache = Array2::<u8>::zeros((n, m));
        let (visited, loops) = path.into_iter().fold(
            (Array2::<u8>::zeros((n, m)), 0),
            |(mut visited, mut loops), (pos, dir)| {
                visited[(pos.0 as usize, pos.1 as usize)] = 1;

                let block = step(pos, dir);
                if !valid(block, map.dim()) {
                    return (visited, loops);
                }

                let on_path = visited[(block.0 as usize, block.1 as usize)] == 1;
                let is_free = map[(block.0 as usize, block.1 as usize)] == b'.';

                if block != start && is_free && !on_path {
                    map[(block.0 as usize, block.1 as usize)] = b'#';

                    let looped = iterate((pos, dir.turn_right()), |&(pos, dir)| {
                        advance(pos, dir, &map)
                    })
                    .take_while(|(pos, _)| valid(*pos, (n, m)))
                    .find(|&(pos, dir)| {
                        if cache[(pos.0 as usize, pos.1 as usize)] & dir.bits() > 0 {
                            return true;
                        }

                        cache[(pos.0 as usize, pos.1 as usize)] |= dir.bits();
                        false
                    })
                    .map(|_| true)
                    .unwrap_or(false);

                    map[(block.0 as usize, block.1 as usize)] = b'.';
                    cache.fill(0);

                    loops += looped as i32;
                }

                (visited, loops)
            },
        );

        (
            visited
                .into_iter()
                .map(|b| b as usize)
                .sum::<usize>()
                .to_string(),
            loops.to_string(),
        )
    }
}
