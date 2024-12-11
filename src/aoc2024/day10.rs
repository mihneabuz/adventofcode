use lib::{helpers, prelude::*};

use hashbrown::HashSet;
use ndarray::Array2;

fn _reach(pos: (i32, i32), map: &Array2<u8>, res: &mut HashSet<(i32, i32)>, count: &mut u32) {
    let (n, m) = map.dim();

    let height = map[(pos.0 as usize, pos.1 as usize)];
    if height == b'9' {
        res.insert(pos);
        *count += 1;
    }

    if pos.0 > 0 && map[(pos.0 as usize - 1, pos.1 as usize)] == height + 1 {
        _reach((pos.0 - 1, pos.1), map, res, count);
    }

    if pos.0 < n as i32 - 1 && map[(pos.0 as usize + 1, pos.1 as usize)] == height + 1 {
        _reach((pos.0 + 1, pos.1), map, res, count);
    }

    if pos.1 > 0 && map[(pos.0 as usize, pos.1 as usize - 1)] == height + 1 {
        _reach((pos.0, pos.1 - 1), map, res, count);
    }

    if pos.1 < m as i32 - 1 && map[(pos.0 as usize, pos.1 as usize + 1)] == height + 1 {
        _reach((pos.0, pos.1 + 1), map, res, count);
    }
}

fn reach(pos: (i32, i32), map: &Array2<u8>) -> (u32, u32) {
    let mut set = HashSet::new();
    let mut count = 0;
    _reach(pos, map, &mut set, &mut count);
    (set.len() as u32, count)
}

pub struct Day10;

impl Challenge for Day10 {
    aoc!(year = 2024, day = 10);

    fn solve(input: String) -> (String, String) {
        let map = helpers::array2::from_str(&input);

        let (res1, res2) = map
            .indexed_iter()
            .filter_map(|(pos, &height)| (height == b'0').then_some(pos))
            .map(|pos| reach((pos.0 as i32, pos.1 as i32), &map))
            .fold((0, 0), |acc, res| (acc.0 + res.0, acc.1 + res.1));

        (res1.to_string(), res2.to_string())
    }
}
