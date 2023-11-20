use lib::aoc;
use lib::challenge::Challenge;

use std::collections::VecDeque;

use ndarray::Array3;

pub struct Day18;

impl Challenge for Day18 {
    aoc!(year = 2022, day = 18);

    fn solve(input: String) -> (String, String) {
        let cubes = input
            .lines()
            .map(|s| {
                let mut coords = s.split(',').map(|d| d.parse::<usize>().unwrap());
                (
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let n = cubes.len();

        let mut res1 = n * 6;
        for i in 0..n {
            for j in i + 1..n {
                let diff0 = (cubes[i].0 as i64 - cubes[j].0 as i64).abs();
                let diff1 = (cubes[i].1 as i64 - cubes[j].1 as i64).abs();
                let diff2 = (cubes[i].2 as i64 - cubes[j].2 as i64).abs();
                if diff0 + diff1 + diff2 == 1 {
                    res1 -= 2;
                }
            }
        }

        let mut bounds = cubes
            .iter()
            .copied()
            .reduce(|(a, b, c), (x, y, z)| (a.max(x), b.max(y), c.max(z)))
            .unwrap();
        bounds = (bounds.0 + 1, bounds.1 + 1, bounds.2 + 1);

        let mut res2 = 0;
        let mut space = Array3::<u8>::zeros([bounds.0, bounds.1, bounds.2]);
        for (x, y, z) in cubes {
            space[[x, y, z]] = 1;

            if x == 0 || x == bounds.0 - 1 {
                res2 += 1;
            }

            if y == 0 || y == bounds.1 - 1 {
                res2 += 1;
            }

            if z == 0 || z == bounds.2 - 1 {
                res2 += 1;
            }
        }

        let mut seen = Array3::<u8>::zeros([bounds.0, bounds.1, bounds.2]);
        let dirs = [
            [0, 0, 1],
            [0, 0, -1],
            [0, 1, 0],
            [0, -1, 0],
            [1, 0, 0],
            [-1, 0, 0],
        ];

        let mut queue = VecDeque::new();
        queue.push_back([0i32, 0i32, 0i32]);
        while let Some([x, y, z]) = queue.pop_front() {
            for [dx, dy, dz] in dirs.map(|[dx, dy, dz]| [x + dx, y + dy, z + dz]) {
                if dx >= 0
                    && dx < bounds.0 as i32
                    && dy >= 0
                    && dy < bounds.1 as i32
                    && dz >= 0
                    && dz < bounds.2 as i32
                {
                    let next = [dx as usize, dy as usize, dz as usize];
                    if space[next] == 1 {
                        res2 += 1;
                    } else if seen[next] == 0 {
                        seen[next] = 1;
                        queue.push_back([dx, dy, dz]);
                    }
                }
            }
        }

        (res1.to_string(), res2.to_string())
    }
}
