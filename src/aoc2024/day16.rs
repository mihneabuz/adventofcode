use std::{cmp::Reverse, collections::BinaryHeap};

use lib::{helpers, prelude::*};

use hashbrown::{HashMap, hash_map::Entry};

type Pos = (i32, i32);
type Node = (Pos, Pos);

fn solve(start: Pos, end: Pos, map: &mut helpers::Map<u8>) -> usize {
    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<Node, (usize, Vec<Node>)> = HashMap::new();

    heap.push((Reverse(0), (start, (0, 1)), (start, (0, 1))));
    while let Some((Reverse(cost), (pos, path), from)) = heap.pop() {
        match costs.entry((pos, path)) {
            Entry::Occupied(mut entry) => {
                if entry.get_mut().0 == cost {
                    entry.get_mut().1.push(from);
                }
                continue;
            }
            Entry::Vacant(empty) => {
                empty.insert((cost, vec![from]));
            }
        }

        if pos == end {
            let mut trace = vec![(pos, path)];
            while let Some((back, dir)) = trace.pop() {
                if back == start {
                    continue;
                }

                map[back] = b'O';
                trace.extend(costs[&(back, dir)].1.iter());
            }

            map[start] = b'O';

            return cost;
        }

        for (dir, neigh) in map.neighs4i(pos) {
            if map[neigh] != b'#' {
                if dir == path {
                    heap.push((Reverse(cost + 1), (neigh, dir), (pos, dir)));
                } else {
                    heap.push((Reverse(cost + 1000), (pos, dir), (pos, path)));
                }
            }
        }
    }

    unreachable!()
}

pub struct Day16;

impl Challenge for Day16 {
    aoc!(year = 2024, day = 16);

    fn solve(input: String) -> (String, String) {
        let mut map = helpers::Map::from_text(&input);

        let find = |b: u8| map.find(|c| *c == b).map(|(i, j)| (i as i32, j as i32));

        let start = find(b'S').unwrap();
        let end = find(b'E').unwrap();

        let res1 = solve(start, end, &mut map);
        let res2 = map.cells().filter(|&(_, &b)| b == b'O').count();

        (res1.to_string(), res2.to_string())
    }
}
