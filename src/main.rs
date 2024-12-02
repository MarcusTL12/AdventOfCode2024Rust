// #![feature(slice_as_chunks, step_trait, concat_bytes)]

use std::{env, fmt::Display, fs::read_to_string, time::Instant};

use home::home_dir;

// mod util;

mod day1;

#[derive(Debug)]
enum TaskResult {
    // Text(String),
    Number(i64),
}

impl Display for TaskResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Text(s) => s.fmt(f),
            Self::Number(n) => n.fmt(f),
        }
    }
}

type Day = [fn(String) -> TaskResult; 2];

const DAYS: &[Day] = &[day1::PARTS];

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
