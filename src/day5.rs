use std::{cmp::Ordering, collections::HashSet};

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn collect_order(s1: &str) -> HashSet<[u8; 2]> {
    s1.lines()
        .map(|l| {
            l.split_once('|')
                .map(|(a, b)| [a.parse().unwrap(), b.parse().unwrap()])
                .unwrap()
        })
        .collect()
}

fn part1(input: String) -> TaskResult {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let order = collect_order(s1);

    let mut v = Vec::new();

    let mut s = 0;

    for l in s2.lines() {
        v.clear();
        v.extend(l.split(',').map(|x| x.parse::<u8>().unwrap()));

        if v.is_sorted_by(|&a, &b| order.contains(&[a, b])) {
            s += v[v.len() / 2] as u32;
        }
    }

    s.into()
}

fn part2(input: String) -> TaskResult {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let order = collect_order(s1);

    let mut v = Vec::new();

    let mut s = 0;

    for l in s2.lines() {
        v.clear();
        v.extend(l.split(',').map(|x| x.parse::<u8>().unwrap()));

        if !v.is_sorted_by(|&a, &b| order.contains(&[a, b])) {
            v.sort_by(|&a, &b| {
                if order.contains(&[a, b]) {
                    Ordering::Less
                } else if order.contains(&[b, a]) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            s += v[v.len() / 2] as u32;
        }
    }

    s.into()
}
