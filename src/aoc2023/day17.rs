use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

use lib::{aoc, challenge::Challenge};

pub struct Day17;

impl Challenge for Day17 {
    aoc!(year = 2023, day = 17);

    fn solve(input: String) -> (String, String) {
        let map: Box<[Vec<u8>]> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect_vec())
            .collect();

        fn solve<C, I>(map: &[Vec<u8>], crucible: C, max_streak: usize) -> u32
        where
            C: Fn(Dir, u8) -> I,
            I: Iterator<Item = Dir>,
        {
            let n = map.len();

            let mut queue = BinaryHeap::new();
            let mut costs = vec![u32::MAX; n * n * max_streak * 4];
            let heuristic = move |x: i32, y: i32| ((n as i32 - 1) * 2 - x - y) as u32;

            let start = Position {
                pos: (0, 0),
                dir: Dir::Right,
                streak: 0,
            };
            start.set_cost(n, &mut costs, 0);
            queue.push((Reverse(heuristic(0, 0)), start));

            while let Some((cost, node)) = queue.pop() {
                if node.pos == (n as i32 - 1, n as i32 - 1) {
                    return cost.0;
                }

                for (x, y, dir) in neighbors(node.clone(), n, &crucible) {
                    let streak = if node.dir == dir { node.streak + 1 } else { 1 };
                    let cost = node.get_cost(n, &costs) + map[x as usize][y as usize] as u32;

                    let next = Position {
                        pos: (x, y),
                        dir,
                        streak,
                    };

                    if cost < next.get_cost(n, &costs) {
                        next.set_cost(n, &mut costs, cost);
                        queue.push((Reverse(cost + heuristic(x, y)), next));
                    }
                }
            }

            unreachable!()
        }

        let fst = solve(&map, crucible, 3);
        let snd = solve(&map, mega_crucible, 10);

        (fst.to_string(), snd.to_string())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Dir {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    pos: (i32, i32),
    dir: Dir,
    streak: u8,
}

impl Position {
    fn index(&self, size: usize) -> usize {
        (self.streak.max(1) - 1) as usize * 4 * size * size
            + (self.dir as u8) as usize * size * size
            + self.pos.0 as usize * size
            + self.pos.1 as usize
    }

    pub fn get_cost(&self, size: usize, costs: &[u32]) -> u32 {
        costs[self.index(size)]
    }

    pub fn set_cost(&self, size: usize, costs: &mut [u32], cost: u32) {
        costs[self.index(size)] = cost;
    }
}

fn neighbors<C, I>(p: Position, size: usize, crucible: C) -> impl Iterator<Item = (i32, i32, Dir)>
where
    C: FnOnce(Dir, u8) -> I,
    I: Iterator<Item = Dir>,
{
    let Position { pos, dir, streak } = p;

    crucible(dir, streak).filter_map(move |dir| match dir {
        Dir::Up => {
            if pos.0 <= 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1, Dir::Up))
            }
        }

        Dir::Down => {
            if pos.0 as usize >= size - 1 {
                None
            } else {
                Some((pos.0 + 1, pos.1, Dir::Down))
            }
        }

        Dir::Left => {
            if pos.1 <= 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1, Dir::Left))
            }
        }

        Dir::Right => {
            if pos.1 as usize >= size - 1 {
                None
            } else {
                Some((pos.0, pos.1 + 1, Dir::Right))
            }
        }
    })
}

fn crucible(dir: Dir, streak: u8) -> impl Iterator<Item = Dir> {
    (match (dir, streak) {
        (Dir::Up, 0..=2) => [Dir::Up, Dir::Left, Dir::Right].as_ref(),
        (Dir::Up, 3..) => [Dir::Left, Dir::Right].as_ref(),

        (Dir::Down, 0..=2) => [Dir::Down, Dir::Left, Dir::Right].as_ref(),
        (Dir::Down, 3..) => [Dir::Left, Dir::Right].as_ref(),

        (Dir::Left, 0..=2) => [Dir::Left, Dir::Up, Dir::Down].as_ref(),
        (Dir::Left, 3..) => [Dir::Up, Dir::Down].as_ref(),

        (Dir::Right, 0..=2) => [Dir::Right, Dir::Up, Dir::Down].as_ref(),
        (Dir::Right, 3..) => [Dir::Up, Dir::Down].as_ref(),
    })
    .iter()
    .copied()
}

fn mega_crucible(dir: Dir, streak: u8) -> impl Iterator<Item = Dir> {
    let mut output = [None, None, None];

    let mut k = 0;
    if streak < 10 {
        output[k] = Some(dir);
        k += 1;
    }

    if streak == 0 || streak >= 4 {
        match dir {
            Dir::Up | Dir::Down => {
                output[k] = Some(Dir::Left);
                output[k + 1] = Some(Dir::Right)
            }
            Dir::Left | Dir::Right => {
                output[k] = Some(Dir::Up);
                output[k + 1] = Some(Dir::Down)
            }
        }
    }

    output.into_iter().flatten()
}
