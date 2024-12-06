use ndarray::{ArrayViewMut2, s};

use crate::{
    Day, TaskResult,
    util::{input_to_grid_owned, input_to_grid_owned_with_newline},
};

pub const PARTS: Day = [part1, part2];

fn preprocess_and_find_start(mut mat: ArrayViewMut2<u8>) -> [usize; 2] {
    let mut pos = [0, 0];

    for ((i, j), x) in mat.indexed_iter_mut() {
        if *x == b'^' {
            pos = [i, j];
        }

        *x = match *x {
            b'#' => 1,
            _ => 0,
        }
    }

    pos
}

fn move_pos(pos: [usize; 2], dir: usize) -> [usize; 2] {
    const DIRS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    [
        (pos[0] as isize + DIRS[dir][0]) as usize,
        (pos[1] as isize + DIRS[dir][1]) as usize,
    ]
}

fn part1(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    let mut pos = preprocess_and_find_start(mat.view_mut());
    let mut dir = 0;

    let mut visited = 0;

    loop {
        if (mat[pos] & 0b11110) == 0 {
            visited += 1;
        }

        mat[pos] |= 1 << (dir + 1);

        while let Some(1) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
            dir = (dir + 1) % 4;
        }

        if let Some(0) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
            pos = move_pos(pos, dir);
        } else {
            break TaskResult::from(visited);
        }
    }
}

fn part2(input: String) -> TaskResult {
    let mut mat_raw = input_to_grid_owned_with_newline(input.into_bytes());

    let w = mat_raw.shape()[1];

    let mut mat = mat_raw.slice_mut(s![.., 0..w - 1]);

    let startpos = preprocess_and_find_start(mat.view_mut());

    let mut path = Vec::new();

    let mut pos = startpos;
    let mut dir = 0;

    loop {
        if (mat[pos] & 0b11110) == 0 {
            path.push(pos);
        }

        mat[pos] |= 1 << (dir + 1);

        while let Some(1) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
            dir = (dir + 1) % 4;
        }

        if let Some(0) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
            pos = move_pos(pos, dir);
        } else {
            break;
        }
    }

    for x in mat_raw.as_slice_mut().unwrap().iter_mut() {
        *x &= 1;
    }

    let mut n_loops = 0;

    for [i, j] in path.into_iter().skip(1) {
        let mut mat = mat_raw.slice_mut(s![.., 0..w - 1]);

        if mat[[i, j]] & 1 != 0 {
            continue;
        }

        mat[[i, j]] |= 1;

        let mut pos = startpos;
        let mut dir = 0;

        let is_loop = loop {
            if (mat[pos] & (1 << (dir + 1))) != 0 {
                break true;
            }

            mat[pos] |= 1 << (dir + 1);

            while let Some(1) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
                dir = (dir + 1) % 4;
            }

            if let Some(0) = mat.get(move_pos(pos, dir)).map(|&x| x & 1) {
                pos = move_pos(pos, dir);
            } else {
                break false;
            }
        };

        n_loops += is_loop as u32;

        mat[[i, j]] = 0;
        for x in mat_raw.as_slice_mut().unwrap().iter_mut() {
            *x &= 1;
        }
    }

    TaskResult::from(n_loops)
}
