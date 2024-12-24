use std::collections::HashMap;

use crate::{Day, TaskResult, bitarray::BitArray, util::display_join};

pub const PARTS: Day = [part1, part2];

const N: usize = 9;

fn parse_input(input: &str) -> (Vec<&str>, Vec<BitArray<N>>) {
    let mut computers = HashMap::new();
    let mut computers_ordered = Vec::new();

    for l in input.lines() {
        let (a, b) = l.split_once('-').unwrap();

        for x in [a, b] {
            let n = computers.len();
            computers.entry(x).or_insert_with(|| {
                computers_ordered.push(x);
                n
            });
        }
    }

    let mut connections = vec![BitArray::new(); computers.len()];

    for l in input.lines() {
        let (a, b) = l.split_once('-').unwrap();

        let a = computers[&a];
        let b = computers[&b];

        connections[a].set(b, true);
        connections[b].set(a, true);
    }

    (computers_ordered, connections)
}

fn part1(input: String) -> TaskResult {
    let _connections = parse_input(&input);

    let mut bits = BitArray::<4>::new();

    bits.set_range(50, 150, true);
    bits.set_range(60, 140, false);

    println!("{bits}");

    let tmp = bits.trues_iter().collect::<Vec<_>>();

    println!("{tmp:?}");

    todo!()
}

fn part2(input: String) -> TaskResult {
    let (computers, connections) = parse_input(&input);

    let mut p = BitArray::<N>::new();
    p.set_range(0, connections.len(), true);

    let tmp = bk(&connections, BitArray::new(), p, BitArray::new());

    let mut ans: Vec<_> = tmp.trues_iter().map(|i| computers[i]).collect();
    ans.sort();

    let mut buf = Vec::new();

    display_join(&mut buf, ans.into_iter(), ',').unwrap();

    TaskResult::generic(String::from_utf8(buf).unwrap())
}

fn bk<const N: usize>(
    connections: &[BitArray<N>],
    r: BitArray<N>,
    mut p: BitArray<N>,
    mut x: BitArray<N>,
) -> BitArray<N> {
    if p.count_ones() == 0 && x.count_ones() == 0 {
        return r;
    }

    let mut max_clique = BitArray::new();

    for v in p.trues_iter() {
        let c = bk(
            connections,
            {
                let mut r = r;
                r.set(v, true);
                r
            },
            p & connections[v],
            x & connections[v],
        );

        p.set(v, false);
        x.set(v, true);

        if c.count_ones() > max_clique.count_ones() {
            max_clique = c;
        }
    }

    max_clique
}
