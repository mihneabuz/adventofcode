use lib::{aoc, challenge::Challenge, helpers::Map};

pub struct Day3;

impl Challenge for Day3 {
    aoc!(year = 2023, day = 3);

    fn solve(input: String) -> (String, String) {
        let map = Map::from_text(&input);

        let (mut fst, mut snd) = (0, 0);
        for ((i, j), &cell) in map.cells() {
            if !is_symbol(cell) {
                continue;
            }

            let mut nums = Vec::new();
            for (ni, nj) in map.neighs8(i, j) {
                if map.get(ni, nj).is_ascii_digit() {
                    nums.push((ni, expand(map.row(ni), nj)));
                }
            }

            nums.sort();
            nums.dedup();

            if cell == b'*' && nums.len() == 2 {
                snd += nums
                    .iter()
                    .map(|&(i, (l, r))| parse(&map.row(i)[l..r + 1]))
                    .product::<usize>();
            }

            fst += nums
                .into_iter()
                .map(|(i, (l, r))| parse(&map.row(i)[l..r + 1]))
                .sum::<usize>();
        }

        (fst.to_string(), snd.to_string())
    }
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn expand(row: &[u8], j: usize) -> (usize, usize) {
    let (mut l, mut r) = (j, j);

    while l > 0 && row[l - 1].is_ascii_digit() {
        l -= 1;
    }

    while r < row.len() - 1 && row[r + 1].is_ascii_digit() {
        r += 1;
    }

    (l, r)
}

fn parse(bytes: &[u8]) -> usize {
    unsafe {
        String::from_utf8_lossy(bytes)
            .parse::<usize>()
            .unwrap_unchecked()
    }
}
