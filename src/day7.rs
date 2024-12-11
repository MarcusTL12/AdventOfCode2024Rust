use rayon::{iter::ParallelIterator, str::ParallelString};

use arrayvec::ArrayVec;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn possible<I: Iterator<Item = u16> + Clone>(
    target: u64,
    acc: u64,
    mut it: I,
) -> bool {
    let Some(next) = it.next() else {
        return target == acc;
    };

    let mulans = acc * (next as u64);
    if mulans <= target && possible(target, mulans, it.clone()) {
        return true;
    }

    possible(target, acc + (next as u64), it)
}

fn part1(input: String) -> TaskResult {
    let ans: u64 = input
        .par_lines()
        .filter_map(|l| {
            let (target, v) = l.split_once(": ").unwrap();

            let target = target.parse().unwrap();

            let buf: ArrayVec<u16, 16> =
                v.split(' ').map(|x| x.parse().unwrap()).collect();

            let mut it = buf.iter().cloned();

            possible(target, it.next().unwrap() as u64, it).then_some(target)
        })
        .sum();

    ans.into()
}

fn ndigits(x: u16) -> u32 {
    if x != 0 { x.ilog10() + 1 } else { 1 }
}

fn cat(mut a: u64, b: u16) -> u64 {
    for _ in 0..ndigits(b) {
        a *= 10;
    }

    a + b as u64
}

fn possible2<I: Iterator<Item = u16> + Clone>(
    target: u64,
    acc: u64,
    mut it: I,
) -> bool {
    let Some(next) = it.next() else {
        return target == acc;
    };

    let mulans = acc * (next as u64);
    if mulans <= target && possible2(target, mulans, it.clone()) {
        return true;
    }

    let catans = cat(acc, next);
    if catans <= target && possible2(target, catans, it.clone()) {
        return true;
    }

    possible2(target, acc + (next as u64), it)
}

fn part2(input: String) -> TaskResult {
    let ans: u64 = input
        .par_lines()
        .filter_map(|l| {
            let (target, v) = l.split_once(": ").unwrap();

            let target = target.parse().unwrap();

            let buf: ArrayVec<u16, 16> =
                v.split(' ').map(|x| x.parse().unwrap()).collect();

            let mut it = buf.iter().cloned();

            possible2(target, it.next().unwrap() as u64, it).then_some(target)
        })
        .sum();

    ans.into()
}
