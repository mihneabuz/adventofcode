use lib::{helpers, prelude::*};

use hashbrown::HashSet;

pub struct Day12;

impl Challenge for Day12 {
    aoc!(year = 2024, day = 12);

    fn solve(input: String) -> (String, String) {
        let map = helpers::Map::from_text(&input);
        let (n, m) = map.dims();

        let mut res1 = 0;
        let mut res2 = 0;
        let mut visited = helpers::Map::new(n, m);
        for (pos, val) in map.cellsi() {
            let mut area = 0;
            let mut perim = 0;
            let mut edges = HashSet::new();

            let mut neighs = vec![pos];
            while let Some(next) = neighs.pop() {
                if visited[next] > 0 {
                    continue;
                }

                visited[next] = 1u8;
                area += 1;

                for dir in helpers::Map::<u8>::D4 {
                    let neigh = (next.0 + dir.0, next.1 + dir.1);
                    if map.geti(neigh).is_some_and(|n| *n == *val) {
                        neighs.push(neigh);
                    } else {
                        edges.insert((next, dir));
                        perim += 1;
                    }
                }
            }

            let mut sides = 0;
            while let Some((edge, dir)) = edges.iter().copied().next() {
                edges.remove(&(edge, dir));

                let lo = (dir.1, -dir.0);
                let mut next = (edge.0 + lo.0, edge.1 + lo.1);
                while edges.remove(&(next, dir)) {
                    next = (next.0 + lo.0, next.1 + lo.1);
                }

                let hi = (-dir.1, dir.0);
                let mut next = (edge.0 + hi.0, edge.1 + hi.1);
                while edges.remove(&(next, dir)) {
                    next = (next.0 + hi.0, next.1 + hi.1);
                }

                sides += 1;
            }

            res1 += area * perim;
            res2 += area * sides;
        }

        (res1.to_string(), res2.to_string())
    }
}
