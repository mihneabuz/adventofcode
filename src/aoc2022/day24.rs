use std::fs;

const WALL: char = '#';

const UP: char = '^';
const DOWN: char = 'v';
const RIGHT: char = '>';
const LEFT: char = '<';

type Pos = (usize, usize);
type Map = Vec<Vec<char>>;

fn neighs((i, j): (usize, usize)) -> [(usize, usize); 5] {
    [(i + 1, j), (i, j + 1), (i, j), (i - 1, j), (i, j - 1)]
}

fn valid(map: &Map, (i, j): Pos, mut iter: usize) -> bool {
    if map[i][j] == WALL {
        return false;
    }

    let n = map.len() - 4;
    let m = map[0].len() - 2;

    let n3 = n * n * n;
    let m3 = m * m * m;

    iter %= n * m;

    if map[2 + (i - 2 + iter) % n][j] == UP {
        return false;
    }

    if map[2 + (n3 + i - 2 - iter) % n][j] == DOWN {
        return false;
    }

    if map[i][1 + (j - 1 + iter) % m] == LEFT {
        return false;
    }

    if map[i][1 + (m3 + j - 1 - iter) % m] == RIGHT {
        return false;
    }

    true
}

fn solve(map: &Map, start: Pos, end: Pos, min: usize) -> usize {
    let mut states = vec![start];
    for iter in min.. {
        let mut next_states = vec![];

        for pos in states.into_iter() {
            for (di, dj) in neighs(pos) {
                if (di, dj) == end {
                    return iter;
                }

                if (di, dj) == start || valid(&map, (di, dj), iter) {
                    next_states.push((di, dj));
                }
            }
        }

        next_states.sort();
        next_states.dedup();

        states = next_states;
    }

    0
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut map: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    map.insert(0, vec![WALL; map[0].len()]);
    map.push(vec![WALL; map[0].len()]);

    let start = (1, 1);
    let end = (map.len() - 2, map[0].len() - 2);

    map[start.0][start.1] = WALL;
    map[end.0][end.1] = WALL;

    let res1 = solve(&map, start, end, 1);
    let tmp = solve(&map, end, start, res1);
    let res2 = solve(&map, start, end, tmp);

    println!("1: {}", res1);
    println!("2: {}", res2);
}
