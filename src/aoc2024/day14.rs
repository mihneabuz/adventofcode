use itertools::Itertools;
use lib::{helpers::unchecked_parse, prelude::*};

pub struct Day14;

impl Challenge for Day14 {
    aoc!(year = 2024, day = 14);

    fn solve(input: String) -> (String, String) {
        let robots = input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once(' ').unwrap();
                let (posx, posy) = pos[2..].split_once(',').unwrap();
                let pos = (unchecked_parse::<i32>(posx), unchecked_parse::<i32>(posy));

                let (velx, vely) = vel[2..].split_once(',').unwrap();
                let vel = (unchecked_parse::<i32>(velx), unchecked_parse::<i32>(vely));

                (pos, vel)
            })
            .collect_vec();

        let bounds = (101, 103);
        let wrap = |(mut x, mut y): (i32, i32)| {
            x %= bounds.0;
            y %= bounds.1;

            if x < 0 {
                x += bounds.0;
            }

            if y < 0 {
                y += bounds.1;
            }

            (x, y)
        };

        let advance = |pos: (i32, i32), vel: (i32, i32), times| {
            wrap((pos.0 + vel.0 * times, pos.1 + vel.1 * times))
        };

        let quadrant = |pos: (i32, i32)| {
            let left = pos.0 < (bounds.0 - 1) / 2;
            let right = pos.0 > (bounds.0 - 1) / 2;

            let up = pos.1 < (bounds.1 - 1) / 2;
            let down = pos.1 > (bounds.1 - 1) / 2;

            if left && up {
                Some(0)
            } else if right && up {
                Some(1)
            } else if left && down {
                Some(2)
            } else if right && down {
                Some(3)
            } else {
                None
            }
        };

        let mut quadrants = [0, 0, 0, 0];
        for robot in robots.iter() {
            let fin = advance(robot.0, robot.1, 100);
            if let Some(quad) = quadrant(fin) {
                quadrants[quad] += 1;
            }
        }
        let res1 = quadrants.into_iter().product::<u32>();

        // let mut robots = robots;
        // for i in 1..10000 {
        //     let mut map = (" ".repeat(bounds.0 as usize) + "\n").repeat(bounds.1 as usize);
        //
        //     for robot in robots.iter_mut() {
        //         robot.0 = advance(robot.0, robot.1, 1);
        //
        //         let pos = robot.0;
        //         unsafe {
        //             map.as_bytes_mut()[pos.0 as usize + (pos.1 * (bounds.0 + 1)) as usize] = b'A'
        //         };
        //     }
        //
        //     println!("{}\n{}", map, i);
        //     let _ = std::io::stdin().read_line(&mut String::new());
        // }

        (res1.to_string(), "manual".to_string())
    }
}
