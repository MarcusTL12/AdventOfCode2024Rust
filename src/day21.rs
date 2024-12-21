use crate::{Day, TaskResult};

pub const PARTS: Day = [part1, part2];

fn n_moves_x_first(
    depth: usize,
    from: [i64; 2],
    to: [i64; 2],
    is_keypad: bool,
) -> Option<i64> {
    let mut dx = to[0] - from[0];
    let mut dy = to[1] - from[1];

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
    while dx != 0 {
        let s = dx.signum();
        dx -= s;

        let [tx, ty] = match s {
            1 => [2, 1],
            -1 => [0, 1],
            _ => unreachable!(),
        };

        n_steps += n_moves(depth - 1, [robot_x, robot_y], [tx, ty], false);
        robot_x = tx;
        robot_y = ty;
    }

    // Do up or down button press using next robot
    while dy != 0 {
        let s = dy.signum();
        dy -= s;

        let [tx, ty] = match s {
            1 => [1, 1],
            -1 => [1, 0],
            _ => unreachable!(),
        };

        n_steps += n_moves(depth - 1, [robot_x, robot_y], [tx, ty], false);
        robot_x = tx;
        robot_y = ty;
    }

    // Move back to 'A' key to press it once
    n_steps += n_moves(depth - 1, [robot_x, robot_y], [2, 0], false);

    Some(n_steps)
}

fn n_moves_y_first(
    depth: usize,
    from: [i64; 2],
    to: [i64; 2],
    is_keypad: bool,
) -> Option<i64> {
    let mut dx = to[0] - from[0];
    let mut dy = to[1] - from[1];

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
    while dy != 0 {
        let s = dy.signum();
        dy -= s;

        let [tx, ty] = match s {
            1 => [1, 1],
            -1 => [1, 0],
            _ => unreachable!(),
        };

        n_steps += n_moves(depth - 1, [robot_x, robot_y], [tx, ty], false);
        robot_x = tx;
        robot_y = ty;
    }

    // Do left or right button press using next robot
    while dx != 0 {
        let s = dx.signum();
        dx -= s;

        let [tx, ty] = match s {
            1 => [2, 1],
            -1 => [0, 1],
            _ => unreachable!(),
        };

        n_steps += n_moves(depth - 1, [robot_x, robot_y], [tx, ty], false);
        robot_x = tx;
        robot_y = ty;
    }

    // Move back to 'A' key to press it once
    n_steps += n_moves(depth - 1, [robot_x, robot_y], [2, 0], false);

    Some(n_steps)
}

fn n_moves(depth: usize, from: [i64; 2], to: [i64; 2], is_keypad: bool) -> i64 {
    if depth == 0 {
        return 1;
    }

    [
        n_moves_x_first(depth, from, to, is_keypad),
        n_moves_y_first(depth, from, to, is_keypad),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap()
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
    input.lines().map(|l| {
        let m: i64 = l.split_at(l.len() - 1).0.parse().unwrap();

        let mut x = 2;
        let mut y = 3;

        let mut n = 0;

        for &key in l.as_bytes() {
            let [tx, ty] = keypad_coords(key);
            n += n_moves(3, [x, y], [tx, ty], true);
            [x, y] = [tx, ty];
        }

        n * m
    }).sum::<i64>().into()
}

fn part2(input: String) -> TaskResult {
    input.lines().map(|l| {
        let m: i64 = l.split_at(l.len() - 1).0.parse().unwrap();

        let mut x = 2;
        let mut y = 3;

        let mut n = 0;

        for &key in l.as_bytes() {
            let [tx, ty] = keypad_coords(key);
            n += n_moves(11, [x, y], [tx, ty], true);
            [x, y] = [tx, ty];
        }

        n * m
    }).sum::<i64>().into()
}
