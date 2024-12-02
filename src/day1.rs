use std::collections::HashMap;

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
    let mut counter1 = HashMap::new();
    let mut counter2 = HashMap::new();

    let mut s = 0;

    for [x, y] in input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<ArrayVec<u64, 2>>()
            .into_inner()
            .unwrap()
    }) {
        counter1.entry(x).and_modify(|c| *c += 1).or_insert(1);
        s += counter2.get(&x).unwrap_or(&0) * x;
        counter2.entry(y).and_modify(|c| *c += 1).or_insert(1);
        s += counter1.get(&y).unwrap_or(&0) * y;
    }

    TaskResult::Number(s as i64)
}
