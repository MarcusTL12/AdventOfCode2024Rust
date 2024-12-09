use std::iter;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

type T = u16;

fn part1(input: String) -> TaskResult {
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
    let ([mut fls, mut spc], _) = input
        .as_bytes()
        .iter()
        .filter_map(|x| {
            x.is_ascii_digit().then_some(x.wrapping_sub(b'0') as usize)
        })
        .enumerate()
        .fold(([const { Vec::new() }; 2], 0), |(mut v, pos), (i, len)| {
            v[i % 2].push((pos as u32, len as u32));
            (v, pos + len)
        });

    for (file_pos, file_len) in fls.iter_mut().rev() {
        for (space_pos, space_len) in
            spc.iter_mut().take_while(|(p, _)| p < file_pos)
        {
            if file_len <= space_len {
                *file_pos = *space_pos;
                *space_pos += *file_len;
                *space_len -= *file_len;
                break;
            }
        }
    }

    fls.into_iter()
        .enumerate()
        .map(|(i, (p, l))| i as u64 * (p..p + l).map(|x| x as u64).sum::<u64>())
        .sum::<u64>()
        .into()
}
