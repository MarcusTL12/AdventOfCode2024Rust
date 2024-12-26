use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn part1(mut input: String) -> TaskResult {
    input.push('\n');
    let data: Vec<u64> = input
        .as_bytes()
        .as_chunks::<43>()
        .0
        .iter()
        .map(|l| {
            l.iter()
                .map(|&x| x == b'#')
                .fold([0, 1], |[acc, base], bit| {
                    [acc | if bit { base } else { 0 }, base << 1]
                })[0]
        })
        .collect();

    data.iter()
        .enumerate()
        .map(|(i, &x)| data.iter().take(i).filter(|&&y| x & y == 0).count())
        .sum::<usize>()
        .into()
}

fn part2(_input: String) -> TaskResult {
    TaskResult::generic("God Jul!")
}
