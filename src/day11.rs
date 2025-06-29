use std::collections::HashMap;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn ndigits(x: u64) -> u32 {
    if x != 0 { x.ilog10() + 1 } else { 1 }
}

fn split_number(n: u64) -> Option<[u64; 2]> {
    let nd = ndigits(n);

    nd.is_multiple_of(2).then(|| {
        let tens = 10u64.pow(nd / 2);

        let a = n / tens;
        let b = n % tens;

        [a, b]
    })
}

fn n_stones(memo: &mut HashMap<(u64, u8), u64>, s: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&x) = memo.get(&(s, depth)) {
        return x;
    }

    let x = if s == 0 {
        n_stones(memo, 1, depth - 1)
    } else if let Some([a, b]) = split_number(s) {
        n_stones(memo, a, depth - 1) + n_stones(memo, b, depth - 1)
    } else {
        n_stones(memo, s * 2024, depth - 1)
    };

    memo.insert((s, depth), x);

    x
}

fn part1(input: String) -> TaskResult {
    let mut memo = HashMap::new();

    input
        .trim_ascii_end()
        .split_ascii_whitespace()
        .map(|x| n_stones(&mut memo, x.parse().unwrap(), 25))
        .sum::<u64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    let mut memo = HashMap::new();

    input
        .trim_ascii_end()
        .split_ascii_whitespace()
        .map(|x| n_stones(&mut memo, x.parse().unwrap(), 75))
        .sum::<u64>()
        .into()
}
