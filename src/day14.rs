use arrayvec::ArrayVec;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{Day, TaskResult, util::crt};

pub const PARTS: Day = [part1, part2];

static REG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

fn part1(input: String) -> TaskResult {
    REG.captures_iter(&input)
        .map(|c| {
            c.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                .zip([101u64, 103].into_iter().cycle())
                .map(|(a, b)| ((a + b as i64) as u64) % b)
                .collect::<ArrayVec<_, 4>>()
                .into_inner()
                .unwrap()
        })
        .fold([0; 4], |mut counts, [x, y, vx, vy]| {
            let x = (x + 100 * vx) % 101;
            let y = (y + 100 * vy) % 103;

            if x != 101 / 2 && y != 103 / 2 {
                let xind = if x < 101 / 2 { 0 } else { 1 };
                let yind = if y < 103 / 2 { 0 } else { 1 };

                counts[xind + 2 * yind] += 1;
            }

            counts
        })
        .into_iter()
        .product::<u64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    let [mut pos, vel] = REG
        .captures_iter(&input)
        .map(|c| {
            c.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                .zip([101u64, 103].into_iter().cycle())
                .map(|(a, b)| ((a + b as i64) as u64) % b)
                .collect::<ArrayVec<_, 4>>()
                .into_inner()
                .unwrap()
        })
        .fold(
            [const { Vec::new() }; 2],
            |[mut pos, mut vel], [x, y, vx, vy]| {
                pos.push([x, y]);
                vel.push([vx, vy]);

                [pos, vel]
            },
        );

    let n = pos.len() as u64;

    let mut minx = (0u64, u64::MAX);
    let mut miny = (0u64, u64::MAX);

    for t in 0..103 {
        let [sx, sy] =
            pos.iter().fold([0, 0], |[sx, sy], [x, y]| [sx + x, sy + y]);
        let [sx2, sy2] = pos
            .iter()
            .fold([0, 0], |[sx2, sy2], [x, y]| [sx2 + x * x, sy2 + y * y]);

        let varx = n * sx2 - sx * sx;
        let vary = n * sy2 - sy * sy;

        if varx < minx.1 {
            minx = (t, varx);
        }

        if vary < miny.1 {
            miny = (t, vary);
        }

        for ([x, y], &[vx, vy]) in pos.iter_mut().zip(&vel) {
            *x = (*x + vx) % 101;
            *y = (*y + vy) % 103;
        }
    }

    crt(minx.0 as i64, 101, miny.0 as i64, 103).into()
}
