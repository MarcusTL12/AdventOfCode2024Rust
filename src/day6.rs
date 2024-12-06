use std::cell::Cell;

use ndarray::ArrayViewMut2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thread_local::ThreadLocal;

use crate::{Day, TaskResult, util::input_to_grid_owned};

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

fn traverse(
    mut mat: ArrayViewMut2<u8>,
    mut pos: [usize; 2],
    capture_path: bool,
    clear: bool,
) -> (usize, Option<Vec<[usize; 2]>>, bool) {
    let mut dir = 0;

    let mut n_visited = 0;
    let mut path = capture_path.then_some(Vec::new());

    let is_loop = loop {
        if (mat[pos] & 0b11110) == 0 {
            n_visited += 1;
            if let Some(v) = &mut path {
                v.push(pos);
            }
        }

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

    if clear {
        for x in mat.as_slice_mut().unwrap().iter_mut() {
            *x &= 1;
        }
    }

    (n_visited, path, is_loop)
}

fn part1(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    let pos = preprocess_and_find_start(mat.view_mut());

    TaskResult::from(traverse(mat.view_mut(), pos, false, false).0)
}

fn part2(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes()).to_owned();

    let startpos = preprocess_and_find_start(mat.view_mut());

    let path = traverse(mat.view_mut(), startpos, true, true).1.unwrap();

    let tls = ThreadLocal::new();

    let n_loops: u32 = path
        .into_par_iter()
        .map(|pos| {
            let cell = tls.get_or(|| Cell::new(Some(mat.clone())));
            let mut mat = cell.replace(None).unwrap();

            mat[pos] |= 1;

            let is_loop = traverse(mat.view_mut(), startpos, false, true).2;

            mat[pos] = 0;

            cell.replace(Some(mat));

            is_loop as u32
        })
        .sum();

    TaskResult::from(n_loops)
}
