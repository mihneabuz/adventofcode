use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use lib::{aoc, challenge::Challenge, helpers::unchecked_parse};

pub struct Day22;

impl Challenge for Day22 {
    aoc!(year = 2023, day = 22);

    fn solve(input: String) -> (String, String) {
        let mut bricks = input.lines().map(parse).collect_vec();
        let n = bricks.len();

        bricks.sort_by_key(|brick| brick.bot());
        for i in 0..n {
            let count = (0..i).fold(bricks[i].pos().2 - 1, |acc, j| {
                if bricks[j].overlaps(&bricks[i]) {
                    acc.min(bricks[i].bot() - bricks[j].top())
                } else {
                    acc
                }
            });

            if count > 0 {
                bricks[i].lower(count);
            }
        }
        bricks.sort_by_key(|brick| brick.bot());

        let mut supporting = vec![(vec![], vec![]); n];
        for i in 0..n {
            for j in 0..i {
                if bricks[j].supports(&bricks[i]) {
                    supporting[i].0.push(j);
                    supporting[j].1.push(i);
                }
            }
        }

        let collapse = |removed: usize| -> usize {
            let mut to_check = VecDeque::from_iter(supporting[removed].1.iter().copied());
            let mut removed: HashSet<usize> = HashSet::from_iter(Some(removed));

            while let Some(idx) = to_check.pop_front() {
                if supporting[idx]
                    .0
                    .iter()
                    .all(|support| removed.contains(support))
                {
                    removed.insert(idx);
                    to_check.extend(supporting[idx].1.iter());
                }
            }

            removed.len() - 1
        };

        let (fst, snd) = (0..n).fold((0, 0), |acc, i| {
            let safe_remove = supporting[i].1.iter().all(|j| supporting[*j].0.len() > 1);
            if safe_remove {
                (acc.0 + 1, acc.1)
            } else {
                (acc.0, acc.1 + collapse(i))
            }
        });

        (fst.to_string(), snd.to_string())
    }
}

type Position = (u32, u32, u32);

#[derive(Clone, Debug)]
enum Brick {
    HorizontalX(Position, u32),
    HorizontalY(Position, u32),
    Verticallll(Position, u32),
}

impl Brick {
    fn pos(&self) -> &Position {
        match self {
            Brick::HorizontalX(pos, _) => pos,
            Brick::HorizontalY(pos, _) => pos,
            Brick::Verticallll(pos, _) => pos,
        }
    }

    fn bot(&self) -> u32 {
        self.pos().2
    }

    fn top(&self) -> u32 {
        match self {
            Brick::HorizontalX(pos, _) => pos.2 + 1,
            Brick::HorizontalY(pos, _) => pos.2 + 1,
            Brick::Verticallll(pos, height) => pos.2 + height + 1,
        }
    }

    fn lower(&mut self, count: u32) {
        let pos = match self {
            Brick::HorizontalX(pos, _) => pos,
            Brick::HorizontalY(pos, _) => pos,
            Brick::Verticallll(pos, _) => pos,
        };
        pos.2 -= count;
    }

    fn overlaps(&self, other: &Self) -> bool {
        let range_overlaps = |(x, lenx), (y, leny)| x <= y + leny && y <= x + lenx;

        match (&self, other) {
            (Brick::HorizontalX(pos1, len1), Brick::HorizontalX(pos2, len2)) => {
                pos1.1 == pos2.1 && range_overlaps((pos1.0, len1), (pos2.0, len2))
            }

            (Brick::HorizontalY(pos1, len1), Brick::HorizontalY(pos2, len2)) => {
                pos1.0 == pos2.0 && range_overlaps((pos1.1, len1), (pos2.1, len2))
            }

            (Brick::Verticallll(pos1, _), Brick::Verticallll(pos2, _)) => {
                (pos1.0, pos1.1) == (pos2.0, pos2.1)
            }

            (Brick::HorizontalX(posx, lenx), Brick::HorizontalY(posy, leny)) => {
                range_overlaps((posx.0, lenx), (posy.0, &0))
                    && range_overlaps((posy.1, leny), (posx.1, &0))
            }

            (Brick::HorizontalY(posy, leny), Brick::HorizontalX(posx, lenx)) => {
                range_overlaps((posx.0, lenx), (posy.0, &0))
                    && range_overlaps((posy.1, leny), (posx.1, &0))
            }

            (Brick::HorizontalX(pos1, len1), Brick::Verticallll(pos2, _)) => {
                pos1.1 == pos2.1 && range_overlaps((pos1.0, len1), (pos2.0, &0))
            }

            (Brick::Verticallll(pos1, _), Brick::HorizontalX(pos2, len2)) => {
                pos1.1 == pos2.1 && range_overlaps((pos2.0, len2), (pos1.0, &0))
            }

            (Brick::HorizontalY(pos1, len1), Brick::Verticallll(pos2, _)) => {
                pos1.0 == pos2.0 && range_overlaps((pos1.1, len1), (pos2.1, &0))
            }

            (Brick::Verticallll(pos1, _), Brick::HorizontalY(pos2, len2)) => {
                pos1.0 == pos2.0 && range_overlaps((pos2.1, len2), (pos1.1, &0))
            }
        }
    }

    fn supports(&self, other: &Self) -> bool {
        self.top() == other.bot() && self.overlaps(other)
    }
}

fn parse(line: &str) -> Brick {
    let (start, end) = line.split_once('~').unwrap();

    let cube = |s: &str| -> Position {
        s.split(',')
            .map(unchecked_parse::<u32>)
            .collect_tuple()
            .unwrap()
    };

    let (start, end) = (cube(start), cube(end));

    if (start.0, start.1) == (end.0, end.1) {
        Brick::Verticallll(
            (start.0, start.1, start.2.min(end.2)),
            start.2.abs_diff(end.2),
        )
    } else if (start.0, start.2) == (end.0, end.2) {
        Brick::HorizontalY(
            (start.0, start.1.min(end.1), start.2),
            start.1.abs_diff(end.1),
        )
    } else {
        Brick::HorizontalX(
            (start.0.min(end.0), start.1, start.2),
            start.0.abs_diff(end.0),
        )
    }
}
