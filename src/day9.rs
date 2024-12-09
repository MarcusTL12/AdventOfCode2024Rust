use std::iter;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    type T = u16;

    const SPACE: T = T::MAX;

    let mut disk: Vec<_> = input
        .as_bytes()
        .iter()
        .filter_map(|x| {
            x.is_ascii_digit().then_some(x.wrapping_sub(b'0') as usize)
        })
        .enumerate()
        .flat_map(|(i, x)| {
            if (i % 2) != 0 {
                iter::repeat_n(SPACE, x)
            } else {
                iter::repeat_n((i / 2) as T, x)
            }
        })
        .collect();

    let mut i = 0;
    while i < disk.len() {
        while i < disk.len() && disk[i] == SPACE {
            disk[i] = disk.pop().unwrap();
        }

        i += 1;
    }

    disk.into_iter()
        .enumerate()
        .map(|(i, x)| i as u64 * x as u64)
        .sum::<u64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
