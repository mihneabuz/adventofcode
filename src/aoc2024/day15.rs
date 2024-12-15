use lib::{helpers, prelude::*};

use itertools::Itertools;

type Pos = (i32, i32);

fn dir(ins: u8) -> Pos {
    match ins {
        b'^' => (-1, 0),
        b'>' => (0, 1),
        b'v' => (1, 0),
        b'<' => (0, -1),
        _ => unreachable!(),
    }
}

fn step(pos: Pos, dir: Pos) -> Pos {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn push_simple(pos: Pos, dir: Pos, map: &mut helpers::Map<u8>) -> bool {
    let next = step(pos, dir);
    match map[next] {
        b'.' => {
            map[next] = map[pos];
            map[pos] = b'.';

            true
        }
        b'O' | b'[' | b']' => {
            let can_push = push_simple(next, dir, map);
            if can_push {
                map[next] = map[pos];
                map[pos] = b'.';
            }
            can_push
        }
        b'#' => false,
        _ => unreachable!(),
    }
}

fn push_complex(pos: Pos, dir: Pos, map: &mut helpers::Map<u8>) -> bool {
    let push = _can_push_complex(pos, dir, map);
    if push {
        _push_complex(pos, dir, map);
    }
    push
}

fn _push_complex(pos: Pos, dir: Pos, map: &mut helpers::Map<u8>) {
    let next = step(pos, dir);
    match map[next] {
        b'[' => {
            let other = (next.0, next.1 + 1);
            _push_complex(next, dir, map);
            _push_complex(other, dir, map);
        }
        b']' => {
            let other = (next.0, next.1 - 1);
            _push_complex(other, dir, map);
            _push_complex(next, dir, map);
        }
        b'.' => {}
        _ => unreachable!(),
    }

    map[next] = map[pos];
    map[pos] = b'.';
}

fn _can_push_complex(pos: Pos, dir: Pos, map: &helpers::Map<u8>) -> bool {
    let next = step(pos, dir);
    match map[next] {
        b'.' => true,
        b'[' => {
            let other = (next.0, next.1 + 1);
            _can_push_complex(next, dir, map) && _can_push_complex(other, dir, map)
        }
        b']' => {
            let other = (next.0, next.1 - 1);
            _can_push_complex(next, dir, map) && _can_push_complex(other, dir, map)
        }
        b'#' => false,
        _ => unreachable!(),
    }
}

pub struct Day15;

impl Challenge for Day15 {
    aoc!(year = 2024, day = 15);

    fn solve(input: String) -> (String, String) {
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let instructions = instructions.bytes().filter(|b| *b != b'\n').collect_vec();

        let mut map = helpers::Map::from_text(map);
        let mut bigmap = helpers::Map::new(map.height(), map.width() * 2);
        map.cells().for_each(|((i, j), byte)| {
            let (b1, b2) = match byte {
                b'#' => (b'#', b'#'),
                b'O' => (b'[', b']'),
                b'.' => (b'.', b'.'),
                b'@' => (b'@', b'.'),
                _ => unreachable!(),
            };

            *bigmap.get_mut(i, 2 * j) = b1;
            *bigmap.get_mut(i, 2 * j + 1) = b2;
        });

        let start = map.find(|b| *b == b'@').unwrap();
        instructions.iter().map(|ins| dir(*ins)).fold(
            (start.0 as i32, start.1 as i32),
            |pos, dir| {
                if push_simple(pos, dir, &mut map) {
                    step(pos, dir)
                } else {
                    pos
                }
            },
        );

        let res1 = map
            .cells()
            .filter(|&(_, byte)| *byte == b'O')
            .map(|(pos, _)| 100 * pos.0 + pos.1)
            .sum::<usize>();

        let start = bigmap.find(|b| *b == b'@').unwrap();
        instructions.iter().map(|ins| dir(*ins)).fold(
            (start.0 as i32, start.1 as i32),
            |pos, dir| {
                let pushed = if dir.1 != 0 {
                    push_simple(pos, dir, &mut bigmap)
                } else {
                    push_complex(pos, dir, &mut bigmap)
                };

                if pushed {
                    step(pos, dir)
                } else {
                    pos
                }
            },
        );

        let res2 = bigmap
            .cells()
            .filter(|&(_, byte)| *byte == b'[')
            .map(|(pos, _)| 100 * pos.0 + pos.1)
            .sum::<usize>();

        (res1.to_string(), res2.to_string())
    }
}
