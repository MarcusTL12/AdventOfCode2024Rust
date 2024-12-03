use arrayvec::ArrayVec;
use regex::Regex;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    TaskResult::from(
        reg.captures_iter(&input)
            .map(|c| {
                c.iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().parse().unwrap())
                    .collect::<ArrayVec<u64, 2>>()
                    .into_inner()
                    .unwrap()
            })
            .map(|[a, b]| a * b)
            .sum::<u64>(),
    )
}

fn part2(input: String) -> TaskResult {
    let reg =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut active = true;
    let mut ans = 0;

    for c in reg.captures_iter(&input) {
        match &c[0] {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    ans += c[1].parse::<u64>().unwrap()
                        * c[2].parse::<u64>().unwrap()
                }
            }
        }
    }

    TaskResult::from(ans)
}
