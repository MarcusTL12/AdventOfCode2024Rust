use std::{collections::HashMap, str::FromStr};

use crate::{Day, TaskResult, util::ConstCollect};

pub const PARTS: Day = [part1, part2];

#[derive(Debug, Clone, Copy)]
enum Operation {
    Or,
    And,
    Xor,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        match s {
            "OR" => Ok(Or),
            "AND" => Ok(And),
            "XOR" => Ok(Xor),
            _ => Err("Illegal Operation!"),
        }
    }
}

impl Operation {
    fn eval(&self, a: bool, b: bool) -> bool {
        use Operation::*;
        match self {
            Or => a | b,
            And => a & b,
            Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Wire<'a> {
    Value(bool),
    Gate(Operation, &'a str, &'a str),
}

fn eval_wire(wires: &mut HashMap<&str, Wire>, w: &str) -> bool {
    match wires[w] {
        Wire::Value(v) => v,
        Wire::Gate(op, a, b) => {
            op.eval(eval_wire(wires, a), eval_wire(wires, b))
        }
    }
}

fn part1(input: String) -> TaskResult {
    let (init, connections) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::new();

    for l in init.lines() {
        let (w, v) = l.split_once(": ").unwrap();

        wires.insert(w, match v {
            "0" => Wire::Value(false),
            "1" => Wire::Value(true),
            _ => panic!(),
        });
    }

    for l in connections.lines() {
        let [a, o, b, _, c] =
            l.split_ascii_whitespace().collect_const().unwrap();

        wires.insert(c, Wire::Gate(o.parse().unwrap(), a, b));
    }

    let mut z_names: Vec<_> = wires
        .keys()
        .cloned()
        .filter(|x| x.as_bytes()[0] == b'z')
        .collect();

    z_names.sort();

    z_names
        .into_iter()
        .map(|w| eval_wire(&mut wires, w))
        .fold([0u64, 1], |[acc, base], bit| {
            [acc + if bit { base } else { 0 }, base * 2]
        })[0]
        .into()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
