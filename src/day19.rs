use std::collections::{HashMap, HashSet};

use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn is_possible(towels: &HashSet<&[u8]>, design: &[u8]) -> bool {
    towels.contains(design)
        || (1..design.len()).any(|i| {
            let (head, tail) = design.split_at(i);

            towels.contains(head) && is_possible(towels, tail)
        })
}

fn part1(input: String) -> TaskResult {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels: HashSet<_> = towels.split(", ").map(|x| x.as_bytes()).collect();
    let designs = designs.as_bytes().par_split(|&x| x == b'\n');

    designs
        .filter(|design| is_possible(&towels, design))
        .count()
        .into()
}

fn count_possible<'a>(
    memo: &mut HashMap<&'a [u8], u64>,
    towels: &HashSet<&[u8]>,
    design: &'a [u8],
) -> u64 {
    if let Some(&c) = memo.get(design) {
        return c;
    }

    let c = if towels.contains(design) { 1 } else { 0 }
        + (1..design.len())
            .filter_map(|i| {
                let (head, tail) = design.split_at(i);

                towels
                    .contains(head)
                    .then(|| count_possible(memo, towels, tail))
            })
            .sum::<u64>();

    memo.insert(design, c);

    c
}

fn part2(input: String) -> TaskResult {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels: HashSet<_> = towels.split(", ").map(|x| x.as_bytes()).collect();
    let designs = designs.as_bytes().split(|&x| x == b'\n');

    let mut memo = HashMap::new();

    designs
        .map(|design| count_possible(&mut memo, &towels, design))
        .sum::<u64>()
        .into()
}
