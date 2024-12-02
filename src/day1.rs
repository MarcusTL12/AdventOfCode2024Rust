use arrayvec::ArrayVec;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let [mut a, mut b] =
        input.lines().fold([const { Vec::new() }; 2], |mut v, l| {
            for (x, v) in l
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .zip(&mut v)
            {
                v.push(x);
            }

            v
        });

    a.sort();
    b.sort();

    TaskResult::Number(
        a.into_iter()
            .zip(b)
            .map(|(x, y)| x.abs_diff(y))
            .sum::<u64>() as i64,
    )
}

fn part2(input: String) -> TaskResult {
    // let mut 

    for [x, y] in input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<ArrayVec<u64, 2>>()
            .into_inner()
            .unwrap()
    }) {

    }

    todo!("{input}")
}
