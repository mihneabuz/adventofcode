use lib::aoc;
use lib::challenge::Challenge;

use std::iter;

use itertools::Itertools;

const EMPTY: char = '.';
const ELF: char = '#';
const ROUNDS: usize = 10;

type Map = Vec<Vec<char>>;

pub struct Day23;

impl Challenge for Day23 {
    aoc!(year = 2022, day = 23);

    fn solve(input: String) -> (String, String) {
        let n = input.lines().next().unwrap().len();
        let pad = n;

        let mut map = iter::repeat(iter::repeat('.').take(pad * 2 + n).collect::<Vec<_>>())
            .take(pad)
            .chain(input.lines().map(|l| {
                iter::repeat('.')
                    .take(pad)
                    .chain(l.chars().chain(iter::repeat('.').take(pad)))
                    .collect::<Vec<_>>()
            }))
            .chain(iter::repeat(iter::repeat('.').take(pad * 2 + n).collect::<Vec<_>>()).take(pad))
            .collect::<Vec<_>>();

        let (mut res1, mut res2) = (0, 0);
        for round in 0.. {
            if round == ROUNDS {
                let b = bounds(&map);
                let count = map.iter().flatten().filter(|&&t| t == ELF).count();
                res1 = (b.1 - b.0 + 1) * (b.3 - b.2 + 1) - count;
            }

            let mut moves = calculate_moves(&map, round);
            if moves.is_empty() {
                res2 = round + 1;
                break;
            }

            apply_moves(&mut map, &mut moves);
        }

        (res1.to_string(), res2.to_string())
    }
}

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

impl Move {
    fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self { from, to }
    }
}

fn neighs(pos: (usize, usize)) -> [(usize, usize); 8] {
    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 - 1, pos.1 - 1),
    ]
}

fn north(pos: (usize, usize)) -> [(usize, usize); 3] {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
    ]
}

fn south(pos: (usize, usize)) -> [(usize, usize); 3] {
    [
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ]
}

fn west(pos: (usize, usize)) -> [(usize, usize); 3] {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1 - 1),
    ]
}

fn east(pos: (usize, usize)) -> [(usize, usize); 3] {
    [
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
    ]
}

fn calculate_moves(map: &Map, round: usize) -> Vec<Move> {
    let (n, m) = (map.len(), map.first().unwrap_or(&vec![]).len());

    let mut res = Vec::new();
    for i in 0..n {
        for j in 0..m {
            let pos = (i, j);
            if map[i][j] == ELF && neighs(pos).into_iter().any(|(i, j)| map[i][j] == ELF) {
                let posibilities = vec![
                    (north(pos), (i - 1, j)),
                    (south(pos), (i + 1, j)),
                    (west(pos), (i, j - 1)),
                    (east(pos), (i, j + 1)),
                ];

                for idx in (0..posibilities.len()).map(|i| (i + round) % posibilities.len()) {
                    let (neighs, new_pos) = posibilities[idx];
                    if neighs.into_iter().all(|(i, j)| map[i][j] == EMPTY) {
                        res.push(Move::new(pos, new_pos));
                        break;
                    }
                }
            }
        }
    }

    res
}

fn apply_moves(map: &mut Map, moves: &mut [Move]) {
    moves.sort_by_key(|m| m.to);
    for (_, mut group) in &moves.iter().chunk_by(|m| m.to) {
        let m = group.next().unwrap();
        if group.next().is_none() {
            map[m.from.0][m.from.1] = EMPTY;
            map[m.to.0][m.to.1] = ELF;
        }
    }
}

fn bounds(map: &Map) -> (usize, usize, usize, usize) {
    let mut res = (
        map.len() / 2,
        map.len() / 2,
        map[0].len() / 2,
        map[0].len() / 2,
    );
    for (i, line) in map.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            if *tile == ELF {
                res = (res.0.min(i), res.1.max(i), res.2.min(j), res.3.max(j));
            }
        }
    }

    res
}
