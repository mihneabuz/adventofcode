use itertools::Itertools;
use lib::{helpers, prelude::*};

pub struct Day4;

impl Challenge for Day4 {
    aoc!(year = 2025, day = 4);

    fn solve(input: String) -> (String, String) {
        let mut map = helpers::Map::from_text(&input);

        let (mut fst, mut snd) = (0, 0);
        loop {
            let remove = map
                .cells()
                .filter_map(|((i, j), value)| {
                    (*value == b'@'
                        && map
                            .neighs8(i, j)
                            .filter(|(ni, nj)| *map.get(*ni, *nj) == b'@')
                            .count()
                            < 4)
                    .then_some((i, j))
                })
                .collect_vec();

            if remove.len() == 0 {
                break;
            }

            if fst == 0 {
                fst = remove.len();
            }

            snd += remove.len();

            for (i, j) in remove {
                map.set(i, j, b'.');
            }
        }

        (fst.to_string(), snd.to_string())
    }
}
