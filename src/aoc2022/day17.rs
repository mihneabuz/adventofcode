use lib::aoc;
use lib::challenge::Challenge;

use std::collections::HashMap;
use std::iter;

const EMPTY_ROW: u8 = 1 << 7;
const PADDING: usize = 7;
const CYCLE_THRESHOLD: usize = 10;

type Rock = Vec<u8>;

#[derive(Clone)]
enum Move {
    Left,
    Right,
}

type Cache = HashMap<(usize, usize), (usize, usize)>;
type Cycle = ((usize, usize), (usize, usize));

pub struct Day17;

impl Challenge for Day17 {
    aoc!(year = 2022, day = 17);

    fn solve(input: String) -> (String, String) {
        let moves = input
            .bytes()
            .filter_map(|b| {
                if b == b'<' {
                    Some(Move::Left)
                } else if b == b'>' {
                    Some(Move::Right)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let rocks = [
            vec!["####"],
            vec![".#.", "###", ".#."],
            vec!["..#", "..#", "###"],
            vec!["#", "#", "#", "#"],
            vec!["##", "##"],
        ]
        .iter()
        .map(|v| from_str_slice(v))
        .collect::<Vec<_>>();

        let fst = solve(&rocks, &moves, 2022);
        let snd = solve(&rocks, &moves, 1000000000000);

        (fst.to_string(), snd.to_string())
    }
}

fn from_str_slice(s: &[&str]) -> Rock {
    s.iter()
        .rev()
        .copied()
        .map(|s| {
            s.chars()
                .enumerate()
                .fold(0, |acc, (i, c)| if c == '#' { acc | 1 << i } else { acc })
        })
        .collect()
}

struct RotVec<T> {
    vec: Vec<T>,
    idx: usize,
}

impl<T> RotVec<T> {
    fn new(vec: Vec<T>) -> Self {
        Self { vec, idx: 0 }
    }

    fn get_next(&mut self) -> &T {
        self.idx += 1;
        &self.vec[(self.idx - 1) % self.vec.len()]
    }

    fn get_idx(&self) -> usize {
        self.idx % self.vec.len()
    }

    fn get_abs_idx(&self) -> usize {
        self.idx
    }
}

fn drop(
    room: &mut Vec<u8>,
    rocks: &mut RotVec<Rock>,
    moves: &mut RotVec<Move>,
    cache: &mut Cache,
) -> (usize, Option<Cycle>) {
    let cycle = {
        let entry = (rocks.get_idx(), moves.get_idx());
        let value = (rocks.get_abs_idx(), room.len() - PADDING);

        cache.insert(entry, value).map(|old| (old, value))
    };

    let rock = rocks.get_next();
    let mut pos = (2, room.len() - 4);

    loop {
        match moves.get_next() {
            Move::Left => {
                if pos.0 > 0
                    && rock
                        .iter()
                        .enumerate()
                        .all(|(i, piece)| piece << (pos.0 - 1) & room[pos.1 + i] == 0)
                {
                    pos.0 -= 1;
                }
            }
            Move::Right => {
                if rock
                    .iter()
                    .enumerate()
                    .all(|(i, piece)| piece << (pos.0 + 1) & room[pos.1 + i] == 0)
                {
                    pos.0 += 1;
                }
            }
        }

        if pos.1 > 0
            && rock
                .iter()
                .enumerate()
                .all(|(i, piece)| piece << pos.0 & room[pos.1 + i - 1] == 0)
        {
            pos.1 -= 1;
            continue;
        }

        break;
    }

    for (i, piece) in rock.iter().enumerate() {
        room[pos.1 + i] |= piece << pos.0
    }

    for (i, &row) in room.iter().rev().take(PADDING).enumerate() {
        if row != EMPTY_ROW {
            room.extend(iter::repeat(EMPTY_ROW).take(PADDING - i));
            break;
        }
    }

    (room.len() - PADDING, cycle)
}

fn solve(rocks: &[Rock], moves: &[Move], n: usize) -> usize {
    let rot_rocks = &mut RotVec::new(rocks.to_owned());
    let rot_moves = &mut RotVec::new(moves.to_owned());

    let mut room = vec![EMPTY_ROW; PADDING];
    let mut cache = HashMap::new();

    let mut heights: Vec<usize> = Vec::new();
    let mut cycles: Vec<Cycle> = Vec::new();

    for _ in 0..n {
        let (height, cycle) = drop(&mut room, rot_rocks, rot_moves, &mut cache);
        heights.push(height);

        if let Some(c) = cycle {
            if let Some(prev) = cycles.last() {
                if prev.0 .0 != c.0 .0 - 1 {
                    cycles.clear();
                }
            }

            cycles.push(c);
            if cycles.len() > CYCLE_THRESHOLD {
                let c = cycles[0];
                return (n - c.0 .0) / (c.1 .0 - c.0 .0) * (c.1 .1 - c.0 .1)
                    + heights[c.0 .0 - 1 + (n - c.0 .0) % (c.1 .0 - c.0 .0)];
            }
        }
    }

    heights[n - 1]
}
