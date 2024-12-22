use std::simd::Simd;

use rayon::{
    iter::ParallelIterator, slice::ParallelSlice, str::ParallelString,
};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn evolve(mut secret: u32) -> u32 {
    secret = ((secret << 6) ^ secret) & 0xffffff;
    secret = ((secret >> 5) ^ secret) & 0xffffff;
    secret = ((secret << 11) ^ secret) & 0xffffff;

    secret
}

const N: usize = 16;

fn evolve_simd(mut secret: Simd<u32, N>) -> Simd<u32, N> {
    secret = ((secret << 6) ^ secret) & Simd::splat(0xffffff);
    secret = ((secret >> 5) ^ secret) & Simd::splat(0xffffff);
    secret = ((secret << 11) ^ secret) & Simd::splat(0xffffff);

    secret
}

fn part1(input: String) -> TaskResult {
    let input: Vec<u32> =
        input.par_lines().map(|l| l.parse().unwrap()).collect();

    input
        .par_chunks(N)
        .map(|c| {
            if c.len() == N {
                let mut v = Simd::from_slice(c);

                for _ in 0..2000 {
                    v = evolve_simd(v);
                }

                v.to_array().into_iter().map(|x| x as u64).sum()
            } else {
                c.iter()
                    .cloned()
                    .map(|mut n| {
                        for _ in 0..2000 {
                            n = evolve(n);
                        }
                        n as u64
                    })
                    .sum::<u64>()
            }
        })
        .sum::<u64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
