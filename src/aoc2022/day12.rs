use std::fs;

const LOWEST: u8 = 'a' as u8;
const START: u8 = 'S' as u8;
const EXIT: u8 = 'E' as u8;
const EXIT_ELEV: u8 = 'z' as u8;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let map = input
        .split_whitespace()
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (n, m) = (map.len(), map[0].len());
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut start = (0, 0);
    let mut exit = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == START {
                start = (i, j);
            }

            if map[i][j] == EXIT {
                exit = (i, j);
            }
        }
    }

    let mut seen = vec![vec![false; m]; n];
    seen[start.0][start.1] = true;
    let mut queue = vec![start];

    let mut iter1 = 1;
    'outer: while queue.len() > 0 {
        let mut next_queue = Vec::new();

        while let Some((i, j)) = queue.pop() {
            for (di, dj) in dirs
                .iter()
                .map(|(dx, dy)| (i as i64 + dx, j as i64 + dy))
                .filter(|(di, dj)| *di >= 0 && *dj >= 0 && *di < n as i64 && *dj < m as i64)
                .map(|(di, dj)| (di as usize, dj as usize))
            {
                if map[di][dj] == EXIT && map[i][j] + 1 >= EXIT_ELEV {
                    break 'outer;
                }

                if !seen[di][dj] && (map[i][j] == START || map[di][dj] <= map[i][j] + 1) {
                    next_queue.push((di, dj));
                    seen[di][dj] = true;
                }
            }
        }

        queue = next_queue;
        iter1 += 1;
    }

    seen = vec![vec![false; m]; n];
    seen[exit.0][exit.1] = true;
    queue = vec![exit];

    let mut iter2 = 1;
    'outer: while queue.len() > 0 {
        let mut next_queue = Vec::new();

        while let Some((i, j)) = queue.pop() {
            for (di, dj) in dirs
                .iter()
                .map(|(dx, dy)| (i as i64 + dx, j as i64 + dy))
                .filter(|(di, dj)| *di >= 0 && *dj >= 0 && *di < n as i64 && *dj < m as i64)
                .map(|(di, dj)| (di as usize, dj as usize))
            {
                if map[di][dj] == LOWEST && map[i][j] <= map[di][dj] + 1 {
                    break 'outer;
                }

                let val = if map[i][j] == EXIT { EXIT_ELEV } else { map[i][j] };

                if !seen[di][dj] && val <= map[di][dj] + 1 {
                    next_queue.push((di, dj));
                    seen[di][dj] = true;
                }
            }
        }

        queue = next_queue;
        iter2 += 1;
    }

    println!("1: {}", iter1);
    println!("2: {}", iter2);
}
