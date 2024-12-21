use itertools::Itertools;
use ndarray::{array, Array1};
use ndarray_linalg::LeastSquaresSvd;

use lib::{aoc, challenge::Challenge, helpers::unchecked_parse};

pub struct Day24;

impl Challenge for Day24 {
    aoc!(year = 2023, day = 24);

    fn solve(input: String) -> (String, String) {
        let hailstones = input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once(" @ ").unwrap();
                let parse = |s: &str| -> (f64, f64, f64) {
                    s.split(", ")
                        .map(|s| s.trim())
                        .map(unchecked_parse::<f64>)
                        .collect_tuple()
                        .unwrap()
                };
                (parse(pos), parse(vel))
            })
            .collect_vec();

        let (min, max) = (200000000000000., 400000000000000.);

        let fst = hailstones
            .iter()
            .tuple_combinations::<(_, _)>()
            .filter(|&((pos1, vel1), (pos2, vel2))| {
                // pos1.0 + vel1.0 * a = pos2.0 + vel2.0 * b
                // pos1.1 + vel1.1 * a = pos1.1 + vel1.1 * b
                //  =>
                // a = (pos2.0 + vel2.0 * b - pos1.0) / vel1.0
                // a = (pos2.1 + vel2.1 * b - pos1.1) / vel1.1
                //  =>
                // a = vel2.0 / vel1.0 * b + (pos2.0 - pos1.0) / vel1.0
                // a = vel2.1 / vel1.1 * b + (pos2.1 - pos1.1) / vel1.1
                //  =>
                // vel2.0 / vel1.0 * b + (pos2.0 - pos1.0) / vel1.0 = vel2.1 / vel1.1 * b + (pos2.1 - pos1.1) / vel1.1
                //  =>
                // b = ((pos2.1 - pos1.1) / vel1.1 - (pos2.0 - pos1.0) / vel1.0) / (vel2.0 / vel1.0 - vel2.1 / vel1.1)

                let b = ((pos2.1 - pos1.1) / vel1.1 - (pos2.0 - pos1.0) / vel1.0)
                    / (vel2.0 / vel1.0 - vel2.1 / vel1.1);
                if b < 0. {
                    return false;
                }

                let a = (pos2.0 + vel2.0 * b - pos1.0) / vel1.0;
                if a < 0. {
                    return false;
                }

                let (x, y) = (pos1.0 + vel1.0 * a, pos1.1 + vel1.1 * a);

                x >= min && x <= max && y >= min && y <= max
            })
            .count();

        // pos[i].0 + vel.0 * t[i] = b.0 + a.0 * t[i]
        // pos[i].1 + vel.1 * t[i] = b.1 + a.1 * t[i]
        // pos[i].2 + vel.2 * t[i] = b.2 + a.2 * t[i]
        //
        // we have 6 + n unknowns (6 from initial position + velocity and n from the times)
        // we have 3 * n ecuasions (3 for each hailstone)
        // we can solve this with only 3 hailstones

        let snd = {
            let ((pos1, vel1), (pos2, vel2), (pos3, vel3), (pos4, vel4)) =
                hailstones.into_iter().take(4).collect_tuple().unwrap();

            let f = move |x: &Array1<f64>| -> Array1<f64> {
                array![
                    pos1.0 + vel1.0 * x[6] - x[0] - x[3] * x[6],
                    pos1.1 + vel1.1 * x[6] - x[1] - x[4] * x[6],
                    pos1.2 + vel1.2 * x[6] - x[2] - x[5] * x[6],
                    //
                    pos2.0 + vel2.0 * x[7] - x[0] - x[3] * x[7],
                    pos2.1 + vel2.1 * x[7] - x[1] - x[4] * x[7],
                    pos2.2 + vel2.2 * x[7] - x[2] - x[5] * x[7],
                    //
                    pos3.0 + vel3.0 * x[8] - x[0] - x[3] * x[8],
                    pos3.1 + vel3.1 * x[8] - x[1] - x[4] * x[8],
                    pos3.2 + vel3.2 * x[8] - x[2] - x[5] * x[8],
                    //
                    pos4.0 + vel4.0 * x[9] - x[0] - x[3] * x[9],
                    pos4.1 + vel4.1 * x[9] - x[1] - x[4] * x[9],
                    pos4.2 + vel4.2 * x[9] - x[2] - x[5] * x[9],
                ]
            };

            let iterate = move |x: Array1<f64>| -> Array1<f64> {
                let row = |i: usize, j: usize, vel: f64| {
                    let mut row = [0.; 12];
                    row[i] = -1.;
                    row[i + 3] = -x[j];
                    row[j] = vel - x[i + 3];
                    row
                };

                let jacob = array![
                    row(0, 6, vel1.0),
                    row(1, 6, vel1.1),
                    row(2, 6, vel1.2),
                    //
                    row(0, 7, vel2.0),
                    row(1, 7, vel2.1),
                    row(2, 7, vel2.2),
                    //
                    row(0, 8, vel3.0),
                    row(1, 8, vel3.1),
                    row(2, 8, vel3.2),
                    //
                    row(0, 9, vel4.0),
                    row(1, 9, vel4.1),
                    row(2, 9, vel4.2),
                ];

                let fx = f(&x);

                x - jacob.least_squares(&fx).unwrap().solution
            };

            let mut x = array![
                (pos1.0 + pos2.0 + pos3.0),
                (pos1.1 + pos2.1 + pos3.1),
                (pos1.2 + pos2.2 + pos3.2),
                (vel1.0 + vel2.0 + vel3.0),
                (vel1.1 + vel2.1 + vel3.1),
                (vel1.2 + vel2.2 + vel3.2),
                (pos1.0 + pos2.0 + pos3.0),
                (pos1.1 + pos2.1 + pos3.1),
                (pos1.2 + pos2.2 + pos3.2),
                (pos1.0 + pos2.0 + pos3.0),
                (pos1.1 + pos2.1 + pos3.1),
                (pos1.2 + pos2.2 + pos3.2),
            ];

            let eps = 1e-5;
            while f(&x).iter().sum::<f64>().abs() > eps {
                x = iterate(x);
            }

            x.into_iter()
                .take(3)
                .map(|x| x.round() as usize)
                .sum::<usize>()
        };

        (fst.to_string(), snd.to_string())
    }
}
