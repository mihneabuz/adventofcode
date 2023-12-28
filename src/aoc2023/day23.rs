use std::collections::VecDeque;

use itertools::Itertools;
use smallvec::SmallVec;

use lib::{aoc, challenge::Challenge, helpers::Bitset};

pub struct Day23;

type Graph = Vec<((usize, usize), Vec<(u32, usize)>)>;

impl Challenge for Day23 {
    aoc!(year = 2023, day = 23);

    fn solve(input: String) -> (String, String) {
        let mut map = input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect_vec();

        let (n, m) = (map.len(), map[0].len());
        map[0][1] = b'#';
        map[n - 1][m - 2] = b'#';

        let (start, end) = ((1, 1), (n - 2, m - 2));

        let fst = {
            let longest_path = |start: (usize, usize), end: (usize, usize)| {
                let mut dists = vec![0; n * m];
                let mut visiting = vec![(end, (end.0 + 1, end.1))];

                while let Some((curr, prev)) = visiting.pop() {
                    let dist = dists[curr.0 * m + curr.1];

                    let neighs = [
                        (curr.0 - 1, curr.1, b'^'),
                        (curr.0 + 1, curr.1, b'v'),
                        (curr.0, curr.1 - 1, b'<'),
                        (curr.0, curr.1 + 1, b'>'),
                    ];

                    for (ni, nj, block) in
                        neighs.into_iter().filter(|(ni, nj, _)| (*ni, *nj) != prev)
                    {
                        if ![b'#', block].contains(&map[ni][nj]) && dists[ni * m + nj] < dist + 1 {
                            dists[ni * m + nj] = dist + 1;
                            visiting.push(((ni, nj), curr));
                        }
                    }
                }

                dists[start.0 * m + start.1]
            };

            longest_path(start, end) + 2
        };

        let snd = {
            let graph = convert(map, start);

            let start = graph.iter().position(|node| node.0 == start).unwrap();
            let end = graph.iter().position(|node| node.0 == end).unwrap();

            fn dfs(idx: usize, end: usize, visited: Bitset<u64>, graph: &Graph) -> Option<u32> {
                if idx == end {
                    return Some(0);
                }

                let mut best = None;
                for &(cost, neigh) in graph[idx].1.iter() {
                    if visited.get(neigh) {
                        continue;
                    }

                    if let Some(rest) = dfs(neigh, end, visited.set(idx), graph) {
                        best = Some(best.unwrap_or(0).max(cost + rest));
                    }
                }

                best
            }

            dfs(start, end, Bitset::new(), &graph).unwrap() + 1
        };

        (fst.to_string(), snd.to_string())
    }
}

fn convert(mut map: Vec<Vec<u8>>, start: (usize, usize)) -> Graph {
    let mut graph = vec![(start, vec![])];
    let mut queue = VecDeque::from_iter(Some((0, start)));

    while let Some((idx, start)) = queue.pop_front() {
        let mut dist = 1;
        let mut pos = start;

        loop {
            map[pos.0][pos.1] = b'#';

            let mut neighs: SmallVec<[_; 3]> = [
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ]
            .into_iter()
            .filter(|&(ni, nj)| map[ni][nj] != b'#')
            .collect();

            if neighs.len() == 1 {
                dist += 1;
                pos = neighs.pop().unwrap();
            } else {
                let new_idx = match graph.iter().position(|node| node.0 == pos) {
                    Some(idx) => idx,
                    None => {
                        let new_idx = graph.len();
                        graph.push((pos, Vec::new()));
                        new_idx
                    }
                };

                graph[idx].1.push((dist, new_idx));
                graph[new_idx].1.push((dist, idx));

                for (ni, nj) in neighs {
                    queue.push_back((new_idx, (ni, nj)));
                }

                break;
            }
        }
    }

    graph
}
