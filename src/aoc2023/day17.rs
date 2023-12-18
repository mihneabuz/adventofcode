use std::cmp::Reverse;

use itertools::Itertools;
use lib::{aoc, challenge::Challenge};
use priority_queue::PriorityQueue;

pub struct Day17;

// TODO: improve this, A* maybe?
impl Challenge for Day17 {
    aoc!(year = 2023, day = 17);

    fn solve(input: String) -> (String, String) {
        let map = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect_vec())
            .collect_vec();

        fn solve<C, I>(map: &[Vec<u8>], crucible: C, max_streak: usize) -> u32
        where
            C: Fn(Dir, u8) -> I,
            I: Iterator<Item = Dir>,
        {
            let n = map.len();

            let mut queue = PriorityQueue::new();
            let mut costs = vec![u32::MAX; n * n * max_streak * 4];

            let start = Position {
                pos: (0, 0),
                dir: Dir::Right,
                count: 0,
            };
            start.set_cost(n, &mut costs, 0);
            queue.push(start, Reverse(0));

            loop {
                let (node, cost) = queue.pop().unwrap();

                if node.pos == (n as i32 - 1, n as i32 - 1) {
                    break cost.0;
                }

                for (x, y, dir) in neighbors(node, n, &crucible) {
                    let count = if node.dir == dir { node.count + 1 } else { 1 };
                    let cost = node.get_cost(n, &costs) + map[x as usize][y as usize] as u32;

                    let new = Position {
                        pos: (x, y),
                        dir,
                        count,
                    };

                    if cost < new.get_cost(n, &costs) {
                        new.set_cost(n, &mut costs, cost);
                        queue.push(new, Reverse(cost));
                    }
                }
            }
        }

        let fst = solve(&map, crucible, 3);
        let snd = solve(&map, mega_crucible, 10);

        (fst.to_string(), snd.to_string())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Dir {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Position {
    pos: (i32, i32),
    dir: Dir,
    count: u8,
}

impl Position {
    fn index(&self, size: usize) -> usize {
        (self.count.max(1) - 1) as usize * 4 * size * size
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

fn neighbors<F, I>(
    curr: Position,
    size: usize,
    crucible: F,
) -> impl Iterator<Item = (i32, i32, Dir)>
where
    F: Fn(Dir, u8) -> I,
    I: Iterator<Item = Dir>,
{
    crucible(curr.dir, curr.count).filter_map(move |dir| {
        (match dir {
            Dir::Up => {
                if curr.pos.0 <= 0 {
                    None
                } else {
                    Some((curr.pos.0 - 1, curr.pos.1))
                }
            }

            Dir::Down => {
                if curr.pos.0 as usize >= size - 1 {
                    None
                } else {
                    Some((curr.pos.0 + 1, curr.pos.1))
                }
            }

            Dir::Left => {
                if curr.pos.1 <= 0 {
                    None
                } else {
                    Some((curr.pos.0, curr.pos.1 - 1))
                }
            }

            Dir::Right => {
                if curr.pos.1 as usize >= size - 1 {
                    None
                } else {
                    Some((curr.pos.0, curr.pos.1 + 1))
                }
            }
        })
        .map(|(x, y)| (x, y, dir))
    })
}

fn crucible(dir: Dir, count: u8) -> impl Iterator<Item = Dir> {
    (match (dir, count) {
        (Dir::Up, 0..=2) => vec![Dir::Up, Dir::Left, Dir::Right],
        (Dir::Up, 3..) => vec![Dir::Left, Dir::Right],

        (Dir::Down, 0..=2) => vec![Dir::Down, Dir::Left, Dir::Right],
        (Dir::Down, 3..) => vec![Dir::Left, Dir::Right],

        (Dir::Left, 0..=2) => vec![Dir::Left, Dir::Up, Dir::Down],
        (Dir::Left, 3..) => vec![Dir::Up, Dir::Down],

        (Dir::Right, 0..=2) => vec![Dir::Right, Dir::Up, Dir::Down],
        (Dir::Right, 3..) => vec![Dir::Up, Dir::Down],
    })
    .into_iter()
}

fn mega_crucible(dir: Dir, count: u8) -> impl Iterator<Item = Dir> {
    (match (dir, count) {
        (Dir::Up, 1..=3) => vec![Dir::Up],
        (Dir::Up, 0 | 4..=9) => vec![Dir::Up, Dir::Left, Dir::Right],
        (Dir::Up, 10..) => vec![Dir::Left, Dir::Right],

        (Dir::Down, 1..=3) => vec![Dir::Down],
        (Dir::Down, 0 | 4..=9) => vec![Dir::Down, Dir::Left, Dir::Right],
        (Dir::Down, 10..) => vec![Dir::Left, Dir::Right],

        (Dir::Left, 1..=3) => vec![Dir::Left],
        (Dir::Left, 0 | 4..=9) => vec![Dir::Left, Dir::Up, Dir::Down],
        (Dir::Left, 10..) => vec![Dir::Up, Dir::Down],

        (Dir::Right, 1..=3) => vec![Dir::Right],
        (Dir::Right, 0 | 4..=9) => vec![Dir::Right, Dir::Up, Dir::Down],
        (Dir::Right, 10..) => vec![Dir::Up, Dir::Down],
    })
    .into_iter()
}
