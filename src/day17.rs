use std::simd::{Mask, Simd, cmp::SimdPartialEq};

use arrayvec::ArrayVec;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use crate::{Day, TaskResult, util::display_join};

pub const PARTS: Day = [part1, part2];

fn parse_input(input: String) -> (u64, Vec<u8>) {
    let reg = Regex::new(
        r#"Register A: (\d+)
Register B: 0
Register C: 0

Program: (.+)"#,
    )
    .unwrap();

    let c = reg.captures(&input).unwrap();

    let a = c[1].parse().unwrap();

    let program = c[2].split(',').map(|x| x.parse().unwrap()).collect();

    (a, program)
}

fn eval_combo(x: u8, reg: [u64; 3]) -> u64 {
    match x {
        0..=3 => x as u64,
        4..=6 => reg[x as usize - 4],
        _ => panic!(),
    }
}

fn part1(input: String) -> TaskResult {
    let (a, program) = parse_input(input);

    let mut ip = 0;

    let mut reg = [a, 0, 0];

    let mut outbuf = Vec::new();

    while let Some(op) = program.get(ip) {
        let operand = program[ip + 1];
        match op {
            0 => reg[0] >>= eval_combo(operand, reg),
            1 => reg[1] ^= operand as u64,
            2 => reg[1] = eval_combo(operand, reg) % 8,
            3 => {
                if reg[0] != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => reg[1] ^= reg[2],
            5 => outbuf.push((eval_combo(operand, reg) % 8) as u8),
            6 => reg[1] = reg[0] >> eval_combo(operand, reg),
            7 => reg[2] = reg[0] >> eval_combo(operand, reg),
            _ => panic!(),
        }

        ip += 2;
    }

    let mut buf = Vec::new();

    display_join(&mut buf, outbuf.into_iter(), ",").unwrap();

    TaskResult::generic(String::from_utf8(buf).unwrap())
}

const N: usize = 8;

#[inline(always)]
fn get_out(a: Simd<u64, N>) -> Simd<u64, N> {
    let mask = Simd::splat(7);

    let a8 = a & mask;

    let shifter = a8 ^ Simd::splat(2);
    let b = a8 ^ Simd::splat(1);
    let c = (a >> shifter) & mask;

    b ^ c
}

#[inline(always)]
fn check_a(program: &[u8], mut a: Simd<u64, N>) -> Mask<i64, N> {
    let mut i = 0;

    let mut status = Mask::splat(true);

    while i < program.len() && status.any() && !a.simd_eq(Simd::splat(0)).all()
    {
        let o = get_out(a);
        status &= o.simd_eq(Simd::splat(program[i] as u64));
        a >>= Simd::splat(3);

        i += 1;
    }

    status & a.simd_eq(Simd::splat(0)) & Mask::splat(i == program.len())
}

fn part2(input: String) -> TaskResult {
    let (_, program) = parse_input(input);

    let strider = Simd::from_array(
        (0..N).collect::<ArrayVec<_, N>>().into_inner().unwrap(),
    );

    let limit: u64 = 38_000_000_000_000;

    let (a, o) = (0..(limit / N as u64))
        .into_par_iter()
        .map(|n| Simd::splat(n * N as u64) + strider)
        .find_map_first(|a| {
            let o = check_a(&program, a);

            o.any().then_some((a, o))
        })
        .unwrap();

    let &a = a.as_array();
    let o = o.to_array();

    let a = a
        .into_iter()
        .zip(o)
        .find_map(|(a, o)| o.then_some(a))
        .unwrap();

    a.into()
}
