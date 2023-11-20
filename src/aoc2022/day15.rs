use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

type Point = (i32, i32);
struct Sensor {
    pos: Point,
    beacon: Point,
}

fn parse_sensor(s: &str) -> Sensor {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    let cap = RE.captures(s).unwrap();

    let pos = (
        cap.get(1).unwrap().as_str().parse().unwrap(),
        cap.get(2).unwrap().as_str().parse().unwrap(),
    );
    let beacon = (
        cap.get(3).unwrap().as_str().parse().unwrap(),
        cap.get(4).unwrap().as_str().parse().unwrap(),
    );

    Sensor { pos, beacon }
}

fn slice_bounds(sen: &Sensor, row: i32) -> Option<(i32, i32)> {
    let dist = (sen.pos.1 - row).abs();
    let range = (sen.pos.0 - sen.beacon.0).abs() + (sen.pos.1 - sen.beacon.1).abs();

    if dist > range {
        None
    } else {
        Some((sen.pos.0 - range + dist, sen.pos.0 + range - dist))
    }
}

fn add_segment(segments: &mut Vec<(i32, i32)>, slice: (i32, i32)) {
    let mut i = 0;
    while i < segments.len() && slice.0 > segments[i].1 {
        i += 1;
    }

    if i == segments.len() {
        segments.push(slice);
    } else if slice.1 < segments[i].0 {
        segments.insert(i, slice);
    } else if slice.0 >= segments[i].0 {
        if slice.1 > segments[i].1 {
            add_segment(segments, (segments[i].1 + 1, slice.1));
        }
    } else if slice.1 > segments[i].0 {
        segments[i].0 = slice.0;
        if slice.1 > segments[i].1 {
            add_segment(segments, (segments[i].1 + 1, slice.1));
        }
    } else {
        segments.insert(i, (slice.0, slice.1 - 1));
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut lines = content.lines();

    let row = lines.next().unwrap().parse::<i32>().unwrap();
    let max = lines.next().unwrap().parse::<i32>().unwrap();
    let sensors = lines.map(parse_sensor).collect::<Vec<_>>();

    let mut beacons = sensors
        .iter()
        .filter(|s| s.beacon.1 == row)
        .map(|s| s.beacon.0)
        .collect::<Vec<_>>();
    beacons.sort();
    beacons.dedup();

    let total = sensors
        .iter()
        .filter_map(|s| slice_bounds(s, row))
        .fold(Vec::new(), |mut segments, slice| {
            add_segment(&mut segments, slice);
            segments
        })
        .iter()
        .map(|(start, end)| *end - *start + 1)
        .sum::<i32>();

    let res1 = total - beacons.len() as i32;

    let mut res2 = 0;
    'outer: for row in 0..=max {
        let mut segments = Vec::new();
        sensors
            .iter()
            .filter_map(|s| slice_bounds(s, row))
            .for_each(|slice| add_segment(&mut segments, slice));

        for window in segments.windows(2) {
            let (prev, curr) = (window[0], window[1]);
            if curr.0 == prev.1 + 2 && prev.1 < max {
                res2 = (prev.1 + 1) as u64 * 4000000 + row as u64;
                break 'outer
            }

            if curr.1 > max {
                break
            }
        }
    }

    println!("1: {}", res1);
    println!("2: {}", res2);
}
