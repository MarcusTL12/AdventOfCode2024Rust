use std::collections::HashMap;

use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn n_moves_x_first(
    memo: &mut HashMap<(usize, [[i64; 2]; 2], bool), i64>,
    depth: usize,
    from: [i64; 2],
    to: [i64; 2],
    is_keypad: bool,
) -> Option<i64> {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];

    if depth == 0 {
        panic!()
    }

    let tmp_target = [to[0], from[1]];

    if is_keypad && tmp_target == [0, 3] || !is_keypad && tmp_target == [0, 0] {
        return None;
    }

    // Need to make the moves using next robot starting on A
    let mut robot_x = 2;
    let mut robot_y = 0;

    let mut n_steps = 0;

    // Do left or right button press using next robot
    if dx != 0 {
        let s = dx.signum();

        let [tx, ty] = match s {
            1 => [2, 1],
            -1 => [0, 1],
            _ => unreachable!(),
        };

        n_steps +=
            n_moves(memo, depth - 1, [robot_x, robot_y], [tx, ty], false)
                + dx.abs()
                - 1;

        robot_x = tx;
        robot_y = ty;
    }

    // Do up or down button press using next robot
    if dy != 0 {
        let s = dy.signum();

        let [tx, ty] = match s {
            1 => [1, 1],
            -1 => [1, 0],
            _ => unreachable!(),
        };

        n_steps +=
            n_moves(memo, depth - 1, [robot_x, robot_y], [tx, ty], false)
                + dy.abs()
                - 1;
        robot_x = tx;
        robot_y = ty;
    }

    // Move back to 'A' key to press it once
    n_steps += n_moves(memo, depth - 1, [robot_x, robot_y], [2, 0], false);

    Some(n_steps)
}

fn n_moves_y_first(
    memo: &mut HashMap<(usize, [[i64; 2]; 2], bool), i64>,
    depth: usize,
    from: [i64; 2],
    to: [i64; 2],
    is_keypad: bool,
) -> Option<i64> {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];

    if depth == 0 {
        panic!()
    }

    let tmp_target = [from[0], to[1]];

    if is_keypad && tmp_target == [0, 3] || !is_keypad && tmp_target == [0, 0] {
        return None;
    }

    // Need to make the moves using next robot starting on A
    let mut robot_x = 2;
    let mut robot_y = 0;

    let mut n_steps = 0;

    // Do up or down button press using next robot
    if dy != 0 {
        let s = dy.signum();

        let [tx, ty] = match s {
            1 => [1, 1],
            -1 => [1, 0],
            _ => unreachable!(),
        };

        n_steps +=
            n_moves(memo, depth - 1, [robot_x, robot_y], [tx, ty], false)
                + dy.abs()
                - 1;

        robot_x = tx;
        robot_y = ty;
    }

    // Do left or right button press using next robot
    if dx != 0 {
        let s = dx.signum();

        let [tx, ty] = match s {
            1 => [2, 1],
            -1 => [0, 1],
            _ => unreachable!(),
        };

        n_steps +=
            n_moves(memo, depth - 1, [robot_x, robot_y], [tx, ty], false)
                + dx.abs()
                - 1;

        robot_x = tx;
        robot_y = ty;
    }

    // Move back to 'A' key to press it once
    n_steps += n_moves(memo, depth - 1, [robot_x, robot_y], [2, 0], false);

    Some(n_steps)
}

fn n_moves(
    memo: &mut HashMap<(usize, [[i64; 2]; 2], bool), i64>,
    depth: usize,
    from: [i64; 2],
    to: [i64; 2],
    is_keypad: bool,
) -> i64 {
    if depth == 0 {
        return 1;
    }

    let k = (depth, [from, to], is_keypad);

    if let Some(&x) = memo.get(&k) {
        return x;
    }

    let x = [
        n_moves_x_first(memo, depth, from, to, is_keypad),
        n_moves_y_first(memo, depth, from, to, is_keypad),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap();

    memo.insert(k, x);

    x
}

fn keypad_coords(key: u8) -> [i64; 2] {
    match key {
        b'0' => [1, 3],
        b'1' => [0, 2],
        b'2' => [1, 2],
        b'3' => [2, 2],
        b'4' => [0, 1],
        b'5' => [1, 1],
        b'6' => [2, 1],
        b'7' => [0, 0],
        b'8' => [1, 0],
        b'9' => [2, 0],
        b'A' => [2, 3],
        _ => panic!(),
    }
}

fn part1(input: String) -> TaskResult {
    let mut memo = HashMap::new();

    input
        .lines()
        .map(|l| {
            let m: i64 = l.split_at(l.len() - 1).0.parse().unwrap();

            let mut x = 2;
            let mut y = 3;

            let mut n = 0;

            for &key in l.as_bytes() {
                let [tx, ty] = keypad_coords(key);
                n += n_moves(&mut memo, 3, [x, y], [tx, ty], true);
                [x, y] = [tx, ty];
            }

            n * m
        })
        .sum::<i64>()
        .into()
}

fn part2(input: String) -> TaskResult {
    let mut memo = HashMap::new();

    input
        .lines()
        .map(|l| {
            let m: i64 = l.split_at(l.len() - 1).0.parse().unwrap();

            let mut x = 2;
            let mut y = 3;

            let mut n = 0;

            for &key in l.as_bytes() {
                let [tx, ty] = keypad_coords(key);
                n += n_moves(&mut memo, 26, [x, y], [tx, ty], true);
                [x, y] = [tx, ty];
            }

            n * m
        })
        .sum::<i64>()
        .into()
}
