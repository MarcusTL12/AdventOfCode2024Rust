use arrayvec::ArrayVec;
use ndarray::ArrayView2;

use crate::{Day, TaskResult, util::input_to_grid_owned};

pub const PARTS: Day = [part1, part2];

fn locate_antennas(mat: ArrayView2<u8>) -> [ArrayVec<[usize; 2], 4>; 256] {
    let mut antennas = [const { ArrayVec::new_const() }; 256];

    for ((i, j), &x) in mat.indexed_iter() {
        if x != b'.' {
            antennas[x as usize].push([i, j]);
        }
    }

    antennas
}

fn part1(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    let antennas = locate_antennas(mat.view());

    for v in antennas {
        for (i, &[i1, j1]) in v.iter().enumerate() {
            for &[i2, j2] in v.iter().take(i) {
                let di = i2 as isize - i1 as isize;
                let dj = j2 as isize - j1 as isize;

                let i = (i2 as isize + di) as usize;
                let j = (j2 as isize + dj) as usize;

                if let Some(x) = mat.get_mut([i, j]) {
                    *x = b'#';
                }

                let i = (i1 as isize - di) as usize;
                let j = (j1 as isize - dj) as usize;

                if let Some(x) = mat.get_mut([i, j]) {
                    *x = b'#';
                }
            }
        }
    }

    TaskResult::from(mat.into_iter().filter(|&x| x == b'#').count())
}

fn part2(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    let antennas = locate_antennas(mat.view());

    for v in antennas {
        for (i, &[i1, j1]) in v.iter().enumerate() {
            for &[mut i2, mut j2] in v.iter().take(i) {
                let di = i2 as isize - i1 as isize;
                let dj = j2 as isize - j1 as isize;

                let mut i1 = i1;
                let mut j1 = j1;

                while let Some(x) = mat.get_mut([i1, j1]) {
                    *x = b'#';
                    i1 = (i1 as isize - di) as usize;
                    j1 = (j1 as isize - dj) as usize;
                }

                while let Some(x) = mat.get_mut([i2, j2]) {
                    *x = b'#';
                    i2 = (i2 as isize + di) as usize;
                    j2 = (j2 as isize + dj) as usize;
                }
            }
        }
    }

    TaskResult::from(mat.into_iter().filter(|&x| x == b'#').count())
}
