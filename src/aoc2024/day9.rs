use itertools::Itertools;
use lib::prelude::*;

pub struct Day9;

#[derive(Debug, Clone)]
enum Block {
    Empty(u8),
    File(u8, u64),
}

impl Block {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    fn size(&self) -> u8 {
        *match self {
            Block::Empty(size) => size,
            Block::File(size, _) => size,
        }
    }

    fn sub_size(&mut self, sub: u8) {
        let old = match self {
            Block::Empty(size) => size,
            Block::File(size, _) => size,
        };

        *old = old.checked_sub(sub).unwrap();
    }
}

impl Challenge for Day9 {
    aoc!(year = 2024, day = 9);

    fn solve(input: String) -> (String, String) {
        let mut blocks = input
            .bytes()
            .enumerate()
            .map(|(i, b)| (i % 2 == 0, b - b'0'))
            .scan(0, |index, (file, size)| {
                Some(if file {
                    let b = Block::File(size, *index);
                    *index += 1;
                    b
                } else {
                    Block::Empty(size)
                })
            })
            .filter(|block| block.size() > 0)
            .collect_vec();

        // this one is ugly :)
        let mut res1 = 0;
        let (mut lo, mut hi) = (0, blocks.len() - 1);
        let (mut clo, mut chi) = (0, 0);
        for pos in 0.. {
            match &blocks[lo] {
                Block::Empty(size) => {
                    while blocks[hi].is_empty() {
                        hi -= 1;
                    }

                    if let Block::File(size, index) = &blocks[hi] {
                        res1 += index * pos;
                        chi += 1;
                        if chi == *size {
                            hi -= 1;
                            chi = 0;
                        }
                    }

                    *size
                }
                Block::File(size, index) => {
                    res1 += index * pos;

                    *size
                }
            };

            clo += 1;
            if clo == blocks[lo].size() {
                lo += 1;
                clo = 0;
            }

            if lo == hi {
                if let Block::File(size, index) = blocks[lo] {
                    let rest = (size - clo - chi) as u64;
                    res1 += index * (rest * pos + rest * (rest + 1) / 2);
                }

                break;
            }
        }

        // this is even worse :)
        let mut hi = blocks.len() - 1;
        while hi > 0 {
            let block = &blocks[hi];
            if let &Block::File(size, index) = block {
                let slot = blocks
                    .iter()
                    .position(|block| block.is_empty() && block.size() >= size);

                if let Some(pos) = slot {
                    if pos > hi {
                        hi -= 1;
                        continue;
                    }

                    if blocks[pos].size() == size {
                        blocks.swap(pos, hi);
                        hi -= 1;
                    } else {
                        blocks[hi] = Block::Empty(size);
                        blocks[pos].sub_size(size);
                        blocks.insert(pos, Block::File(size, index));
                    }
                } else {
                    hi -= 1;
                }
            } else {
                hi -= 1;
            }
        }

        let mut res2 = 0;
        let mut clo = 0;
        let mut lo = 0;
        for pos in 0.. {
            if lo >= blocks.len() {
                break;
            }

            if let Block::File(_, index) = blocks[lo] {
                res2 += index * pos;
            }

            clo += 1;
            if clo == blocks[lo].size() {
                lo += 1;
                clo = 0;
            }
        }

        (res1.to_string(), res2.to_string())
    }
}
