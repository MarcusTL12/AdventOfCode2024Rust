use arrayvec::ArrayVec;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

static REG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)"#,
    )
    .unwrap()
});

fn parse_input(input: &str) -> impl Iterator<Item = [i64; 6]> {
    REG.captures_iter(input).map(|c| {
        c.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse().unwrap())
            .collect::<ArrayVec<_, 6>>()
            .into_inner()
            .unwrap()
    })
}

fn solve_system([a, c, b, d, b1, b2]: [i64; 6]) -> Option<[i64; 2]> {
    let det = a * d - b * c;

    let n1 = d * b1 - b * b2;
    let n2 = -c * b1 + a * b2;

    (n1 % det == 0 && n2 % det == 0).then(|| [n1 / det, n2 / det])
}

fn part1(input: String) -> TaskResult {
    parse_input(&input)
        .filter_map(solve_system)
        .map(|[x1, x2]| 3 * x1 + x2)
        .sum::<i64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    parse_input(&input)
        .map(|mut x| {
            x[4] += 10000000000000;
            x[5] += 10000000000000;
            x
        })
        .filter_map(solve_system)
        .map(|[x1, x2]| 3 * x1 + x2)
        .sum::<i64>()
        .into()
}
