use std::cmp::Reverse;

use itertools::Itertools;
use lib::{aoc, challenge::Challenge};
use priority_queue::PriorityQueue;

pub struct Day17;

// TODO: optimize this
impl Challenge for Day17 {
    aoc!(year = 2023, day = 17);

    fn solve(input: String) -> (String, String) {
        let map = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect_vec())
            .collect_vec();

        fn solve<C>(map: &[Vec<u8>], crucible: C, max_streak: usize) -> u32
        where
            C: Fn(Dir, u8) -> [Option<Dir>; 3],
        {
            let n = map.len();

            let mut queue = PriorityQueue::new();
            let mut costs = vec![u32::MAX; n * n * max_streak * 4];
            let heuristic = |x: i32, y: i32| ((n as i32 - 1) * 2 - x - y) as u32;

            let start = Position {
                pos: (0, 0),
                dir: Dir::Right,
                streak: 0,
            };
            start.set_cost(n, &mut costs, 0);
            queue.push(start, Reverse(heuristic(0, 0)));

            loop {
                let (node, cost) = queue.pop().unwrap();

                if node.pos == (n as i32 - 1, n as i32 - 1) {
                    break cost.0;
                }

                for (x, y, dir) in neighbors(node, n, &crucible) {
                    let streak = if node.dir == dir { node.streak + 1 } else { 1 };
                    let cost = node.get_cost(n, &costs) + map[x as usize][y as usize] as u32;

                    let next = Position {
                        pos: (x, y),
                        dir,
                        streak,
                    };

                    if cost < next.get_cost(n, &costs) {
                        next.set_cost(n, &mut costs, cost);
                        queue.push(next, Reverse(cost + heuristic(x, y)));
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

fn neighbors<C>(curr: Position, size: usize, crucible: C) -> impl Iterator<Item = (i32, i32, Dir)>
where
    C: Fn(Dir, u8) -> [Option<Dir>; 3],
{
    crucible(curr.dir, curr.streak)
        .into_iter()
        .flatten()
        .filter_map(move |dir| {
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

fn crucible(dir: Dir, streak: u8) -> [Option<Dir>; 3] {
    match (dir, streak) {
        (Dir::Up, 0..=2) => [Some(Dir::Up), Some(Dir::Left), Some(Dir::Right)],
        (Dir::Up, 3..) => [Some(Dir::Left), Some(Dir::Right), None],

        (Dir::Down, 0..=2) => [Some(Dir::Down), Some(Dir::Left), Some(Dir::Right)],
        (Dir::Down, 3..) => [Some(Dir::Left), Some(Dir::Right), None],

        (Dir::Left, 0..=2) => [Some(Dir::Left), Some(Dir::Up), Some(Dir::Down)],
        (Dir::Left, 3..) => [Some(Dir::Up), Some(Dir::Down), None],

        (Dir::Right, 0..=2) => [Some(Dir::Right), Some(Dir::Up), Some(Dir::Down)],
        (Dir::Right, 3..) => [Some(Dir::Up), Some(Dir::Down), None],
    }
}

fn mega_crucible(dir: Dir, streak: u8) -> [Option<Dir>; 3] {
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

    output
}
