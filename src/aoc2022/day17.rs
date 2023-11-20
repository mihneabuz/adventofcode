use std::fs;
use std::iter;
use std::collections::HashMap;

const EMPTY_ROW: u8 = 1 << 7;
const PADDING: usize = 7;
const CYCLE_THRESHOLD: usize = 10;

type Rock = Vec<u8>;

#[derive(Clone)]
enum Move {
    Left,
    Right
}

type Cache = HashMap<(usize, usize), (usize, usize)>;
type Cycle = ((usize, usize), (usize, usize));

fn from_str_slice(s: &[&str]) -> Rock {
    s.iter().rev().copied().map(|s| {
        s.chars().enumerate().fold(0, |acc, (i, c)| {
            if c == '#' {
                acc | 1 << i
            } else {
                acc
            }
        })
    }).collect()
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
        self.idx = self.idx + 1;
        &self.vec[(self.idx - 1) % self.vec.len()]
    }

    fn get_idx(&self) -> usize {
        self.idx % self.vec.len()
    }

    fn get_abs_idx(&self) -> usize {
        self.idx
    }
}

fn drop(room: &mut Vec<u8>, rocks: &mut RotVec<Rock>, moves: &mut RotVec<Move>, cache: &mut Cache) -> (usize, Option<Cycle>) {
    let cycle = {
        let entry = (rocks.get_idx(), moves.get_idx());
        let value = (rocks.get_abs_idx(), room.len() - PADDING);

        if let Some(old) = cache.insert(entry, value) {
            Some((old, value))
        } else {
            None
        }
    };

    let rock = rocks.get_next();
    let mut pos = (2, room.len() - 4);

    loop {
        match moves.get_next() {
            Move::Left => {
                if pos.0 > 0 && rock.iter().enumerate().all(
                    |(i, piece)| piece << (pos.0 - 1) & room[pos.1 + i] == 0
                ) {
                    pos.0 -= 1;
                }
            },
            Move::Right => {
                if rock.iter().enumerate().all(
                    |(i, piece)| piece << (pos.0 + 1) & room[pos.1 + i] == 0
                ) {
                    pos.0 += 1;
                }
            },
        }

        if pos.1 > 0 && rock.iter().enumerate().all(
            |(i, piece)| piece << pos.0 & room[pos.1 + i - 1] == 0
        ) {
            pos.1 -= 1;
            continue
        }

        break
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

fn solve(rocks: &Vec<Rock>, moves: &Vec<Move>, n: usize) -> usize {
    let rot_rocks = &mut RotVec::new(rocks.clone());
    let rot_moves = &mut RotVec::new(moves.clone());

    let mut room = vec![EMPTY_ROW; PADDING];
    let mut cache = HashMap::new();

    let mut heights: Vec<usize> = Vec::new();
    let mut cycles: Vec<Cycle> = Vec::new();

    for _ in 0..n {
        let (height, cycle) = drop(&mut room, rot_rocks, rot_moves, &mut cache);
        heights.push(height);

        if let Some(c) = cycle {
            if let Some(prev) = cycles.last() {
                if prev.0.0 != c.0.0 - 1 {
                    cycles.clear();
                }
            }

            cycles.push(c);
            if cycles.len() > CYCLE_THRESHOLD {
                let c = cycles[0];
                return (n - c.0.0) / (c.1.0 - c.0.0) * (c.1.1 - c.0.1) + heights[c.0.0 - 1 + (n - c.0.0) % (c.1.0 - c.0.0)];
            }
        }
    }

    heights[n - 1]
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let moves = content.bytes().filter_map(|b| {
        if b == '<' as u8 {
            Some(Move::Left)
        } else if b == '>' as u8 {
            Some(Move::Right)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let rocks = [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ].iter().map(|v| from_str_slice(v)).collect::<Vec<_>>();

    println!("1: {}", solve(&rocks, &moves, 2022));
    println!("2: {}", solve(&rocks, &moves, 1000000000000));
}
