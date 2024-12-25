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
    Waiting,
}

fn eval_wire(wires: &mut HashMap<&str, Wire>, w: &str) -> bool {
    match wires[w] {
        Wire::Value(v) => v,
        Wire::Gate(op, a, b) => {
            let ans = op.eval(eval_wire(wires, a), eval_wire(wires, b));
            if let Some(x) = wires.get_mut(w) {
                *x = Wire::Value(ans);
            }
            ans
        }
        _ => panic!(),
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

fn _eval_wire_fallable(
    wires: &mut HashMap<&str, Wire>,
    w: &str,
) -> Option<bool> {
    match wires[w] {
        Wire::Value(v) => Some(v),
        Wire::Gate(op, a, b) => {
            if let Some(x) = wires.get_mut(w) {
                *x = Wire::Waiting;
            }
            let a = _eval_wire_fallable(wires, a)?;
            let b = _eval_wire_fallable(wires, b)?;
            let ans = op.eval(a, b);
            if let Some(x) = wires.get_mut(w) {
                *x = Wire::Value(ans);
            }
            Some(ans)
        }
        Wire::Waiting => None,
    }
}

fn eval_wire_tracking<'a>(
    wires: &mut HashMap<&str, Wire<'a>>,
    w: &'a str,
    tracker: &mut Vec<&'a str>,
) -> Option<bool> {
    match wires[w] {
        Wire::Value(v) => Some(v),
        Wire::Gate(op, a, b) => {
            tracker.push(w);
            if let Some(x) = wires.get_mut(w) {
                *x = Wire::Waiting;
            }
            let a = eval_wire_tracking(wires, a, tracker)?;
            let b = eval_wire_tracking(wires, b, tracker)?;
            let ans = op.eval(a, b);
            if let Some(x) = wires.get_mut(w) {
                *x = Wire::Value(ans);
            }
            Some(ans)
        }
        Wire::Waiting => None,
    }
}

fn test_swaps<'a>(
    x: u64,
    y: u64,
    mut wires: HashMap<&'a str, Wire<'a>>,
    _swaps: &[[&str; 2]],
) -> Option<(usize, Vec<Vec<&'a str>>)> {
    let mut z = x + y;

    let mut x_names: Vec<_> = wires
        .keys()
        .cloned()
        .filter(|x| x.as_bytes()[0] == b'x')
        .collect();

    x_names.sort();

    let mut y_names: Vec<_> = wires
        .keys()
        .cloned()
        .filter(|x| x.as_bytes()[0] == b'x')
        .collect();

    y_names.sort();

    // for x_name in x_names {
    //     *wires.entry(x_name).or_insert(Wire::Waiting) = Wire::Value(x & 1 == 1);
    //     x >>= 1;
    // }

    // for y_name in y_names {
    //     *wires.entry(y_name).or_insert(Wire::Waiting) = Wire::Value(y & 1 == 1);
    //     y >>= 1;
    // }

    let mut z_names: Vec<_> = wires
        .keys()
        .cloned()
        .filter(|x| x.as_bytes()[0] == b'z')
        .collect();

    z_names.sort();

    let mut tracker = Vec::new();

    for (i, z_name) in z_names.into_iter().enumerate() {
        let target_bit = z & 1 == 1;
        z >>= 1;

        let mut cur_tracker = Vec::new();

        let ans = eval_wire_tracking(&mut wires, z_name, &mut cur_tracker);

        tracker.push(cur_tracker);

        if ans != Some(target_bit) {
            return Some((i, tracker));
        }
    }

    todo!()
}

fn part2(input: String) -> TaskResult {
    let w1 = "z09";
    let w2 = "rkf";

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

    // if let [Some(x), Some(y)] = [wires.remove(w1), wires.remove(w2)] {
    //     wires.insert(w1, y);
    //     wires.insert(w2, x);
    // }

    let tmp = test_swaps(23854477729455, 22066456577055, wires, &[[w1, w2]]);

    println!("{tmp:?}");

    todo!()
}
