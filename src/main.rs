#![feature(portable_simd, iter_array_chunks, iter_map_windows, slice_as_chunks)]

use std::{env, fmt::Display, fs::read_to_string, time::Instant};

use home::home_dir;

pub mod bitarray;

mod util;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

enum TaskResult {
    Number(i64),
    Generic(Box<dyn Display>),
}

impl Display for TaskResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => n.fmt(f),
            Self::Generic(s) => s.fmt(f),
        }
    }
}

impl TaskResult {
    fn generic<T: Display + 'static>(x: T) -> TaskResult {
        Self::Generic(Box::new(x))
    }
}

impl<T: TryInto<i64> + Clone + Display + 'static> From<T> for TaskResult
where
    T::Error: std::fmt::Debug,
{
    fn from(value: T) -> Self {
        if let Ok(n) = value.clone().try_into() {
            Self::Number(n)
        } else {
            Self::generic(value)
        }
    }
}

type Day = [fn(String) -> TaskResult; 2];

const DAYS: &[Day] = &[
    day1::PARTS,
    day2::PARTS,
    day3::PARTS,
    day4::PARTS,
    day5::PARTS,
    day6::PARTS,
    day7::PARTS,
    day8::PARTS,
    day9::PARTS,
    day10::PARTS,
    day11::PARTS,
    day12::PARTS,
    day13::PARTS,
    day14::PARTS,
    day15::PARTS,
    day16::PARTS,
    day17::PARTS,
    day18::PARTS,
    day19::PARTS,
    day20::PARTS,
    day21::PARTS,
    day22::PARTS,
    day23::PARTS,
    day24::PARTS,
    day25::PARTS,
];

fn load_input(day: usize, example: usize) -> String {
    let path = home_dir()
        .unwrap()
        .join("aoc-input/2024")
        .join(format!("day{day}"))
        .join(if example == 0 {
            "input".to_string()
        } else {
            format!("ex{example}")
        });

    read_to_string(path).unwrap()
}

fn main() {
    let mut args = env::args();

    args.next();

    let quest: usize = args
        .next()
        .expect("Give day number as first cli argument")
        .parse()
        .expect("Day number not numeric");

    let part: usize = args
        .next()
        .expect("Give part as second cli argument")
        .parse()
        .expect("Part not numeric");

    let example: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(0);

    let t = Instant::now();

    let input = load_input(quest, example);

    let t_load = t.elapsed();

    println!("Loading input took: {t_load:?}");

    let t = Instant::now();

    let result = DAYS[quest - 1][part - 1](input);

    let t_solve = t.elapsed();

    println!("{result}");

    println!("Solving took: {t_solve:?}");
}
