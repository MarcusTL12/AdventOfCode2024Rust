use std::{collections::HashMap, str::FromStr};

use arrayvec::ArrayVec;

use crate::{
    Day, TaskResult,
    util::{ConstCollect, display_join},
};

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
    mut x: u64,
    mut y: u64,
    mut wires: HashMap<&'a str, Wire<'a>>,
    swaps: &[[&'a str; 2]],
) -> (Option<usize>, Vec<Vec<&'a str>>) {
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
        .filter(|x| x.as_bytes()[0] == b'y')
        .collect();

    y_names.sort();

    for x_name in x_names {
        *wires.entry(x_name).or_insert(Wire::Waiting) = Wire::Value(x & 1 == 1);
        x >>= 1;
    }

    for y_name in y_names {
        *wires.entry(y_name).or_insert(Wire::Waiting) = Wire::Value(y & 1 == 1);
        y >>= 1;
    }

    for &[w1, w2] in swaps {
        if let [Some(x), Some(y)] = [wires.remove(w1), wires.remove(w2)] {
            wires.insert(w1, y);
            wires.insert(w2, x);
        }
    }

    let mut z_names: Vec<_> = wires
        .keys()
        .cloned()
        .filter(|x| x.as_bytes()[0] == b'z')
        .collect();

    z_names.sort();

    let mut tracker = Vec::new();

    let mut first_wrong = None;

    for (i, z_name) in z_names.into_iter().enumerate() {
        let target_bit = z & 1 == 1;
        z >>= 1;

        let mut cur_tracker = Vec::new();

        let ans = eval_wire_tracking(&mut wires, z_name, &mut cur_tracker);

        tracker.push(cur_tracker);

        if ans != Some(target_bit) && first_wrong.is_none() {
            first_wrong = Some(i);
        }
    }

    (first_wrong, tracker)
}

fn find_swaps<'a>(wires: HashMap<&'a str, Wire<'a>>) -> [[&'a str; 2]; 4] {
    let mut swaps = ArrayVec::<_, 4>::new();

    let x = 23854477729455;
    let y = 22066456577055;

    'outer: for _ in 0..4 {
        let (Some(i), trackers) = test_swaps(x, y, wires.clone(), &swaps)
        else {
            panic!("Fixed with fewer than 4 swaps!")
        };

        let mut best_swap = None;
        let mut best_run = 0;

        let test_x = [
            22066456577055,
            13871334341746,
            13130735162020,
            17044781510614,
            9393861145815,
            8615909262142,
            12730517037478,
            5110479646163,
            9780287576402,
            4707999276817,
            15649038787864,
        ];
        let test_y = [
            22066456577055,
            9221127189547,
            10538398895108,
            10722905323482,
            8449584966808,
            11666591489521,
            677638463355,
            12271188538356,
            10536989812034,
            12732965926479,
            12496496257127,
        ];

        for &w1 in &trackers[i] {
            for &w2 in &trackers[i + 1] {
                swaps.push([w1, w2]);

                if let Some(j) = test_x
                    .into_iter()
                    .zip(test_y.into_iter())
                    .filter_map(|(x, y)| {
                        test_swaps(x, y, wires.clone(), &swaps).0
                    })
                    .min()
                {
                    if j > best_run {
                        best_swap = Some([w1, w2]);
                        best_run = j;
                    }
                } else {
                    break 'outer;
                }

                swaps.pop();
            }
        }

        swaps.push(best_swap.unwrap());
    }

    swaps.into_inner().unwrap()
}

fn part2(input: String) -> TaskResult {
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

    let mut swaps = find_swaps(wires);
    let flattened = swaps.as_flattened_mut();
    flattened.sort();

    let mut buf = Vec::new();

    display_join(&mut buf, flattened.iter(), ',').unwrap();

    TaskResult::generic(String::from_utf8(buf).unwrap())
}
