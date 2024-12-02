use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn is_safe<I: Iterator<Item = u8>>(mut it: I) -> bool {
    let mut inc = true;
    let mut dec = true;

    let mut last = it.next().unwrap();

    for x in it {
        let small = x.abs_diff(last) <= 3;

        inc &= x > last && small;
        dec &= x < last && small;

        if !(inc || dec) {
            break;
        }

        last = x;
    }

    inc ^ dec
}

fn part1(input: String) -> TaskResult {
    TaskResult::from(
        input
            .lines()
            .filter(|l| {
                is_safe(l.split_ascii_whitespace().map(|x| x.parse().unwrap()))
            })
            .count(),
    )
}

fn part2(input: String) -> TaskResult {
    let mut v = Vec::new();

    TaskResult::from(
        input
            .lines()
            .filter(|l| {
                v.clear();
                v.extend(
                    l.split_ascii_whitespace()
                        .map(|x| x.parse::<u8>().unwrap()),
                );

                is_safe(v.iter().cloned())
                    || (0..v.len()).any(|i| {
                        is_safe(
                            v.iter()
                                .enumerate()
                                .filter_map(|(j, &x)| (i != j).then_some(x)),
                        )
                    })
            })
            .count(),
    )
}
