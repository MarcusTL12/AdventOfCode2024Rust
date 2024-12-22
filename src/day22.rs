use std::simd::Simd;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn evolve(mut secret: u32) -> u32 {
    secret = ((secret << 6) ^ secret) & 0xffffff;
    secret = ((secret >> 5) ^ secret) & 0xffffff;
    secret = ((secret << 11) ^ secret) & 0xffffff;

    secret
}

const N: usize = 64;

fn evolve_simd(mut secret: Simd<u32, N>) -> Simd<u32, N> {
    secret = ((secret << 6) ^ secret) & Simd::splat(0xffffff);
    secret = ((secret >> 5) ^ secret) & Simd::splat(0xffffff);
    secret = ((secret << 11) ^ secret) & Simd::splat(0xffffff);

    secret
}

fn part1(input: String) -> TaskResult {
    let mut it = input.lines().map(|l| l.parse().unwrap()).array_chunks();

    let main_sum = it
        .by_ref()
        .flat_map(|n| {
            let mut n = Simd::from_array(n);
            for _ in 0..2000 {
                n = evolve_simd(n);
            }
            n.to_array()
        })
        .map(|x| x as u64)
        .sum::<u64>();

    let rest_sum = it
        .into_remainder()
        .map(|rest| {
            rest.map(|mut n| {
                for _ in 0..2000 {
                    n = evolve(n);
                }
                n as u64
            })
            .sum::<u64>()
        })
        .unwrap_or(0);

    (main_sum + rest_sum).into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
