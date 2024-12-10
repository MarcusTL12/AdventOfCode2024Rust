use ndarray::{Array2, ArrayView2};

use crate::{Day, TaskResult, util::input_to_grid_owned};

pub const PARTS: Day = [part1, part2];

fn rec1(mat: &mut Array2<u8>, pos: [usize; 2]) {
    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    let h1 = mat[pos] & 0b0111_1111;

    if h1 == 9 {
        mat[pos] |= 0b1000_0000;
        return;
    }

    for [di, dj] in DIRS {
        let npos = [
            (pos[0] as isize + di) as usize,
            (pos[1] as isize + dj) as usize,
        ];

        if let Some(1) = mat.get(npos).map(|h2| h2.wrapping_sub(h1)) {
            rec1(mat, npos);
        }
    }
}

fn part1(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    for x in mat.iter_mut() {
        *x = match *x {
            b'0'..=b'9' => *x - b'0',
            _ => 0b0100_0000,
        }
    }

    let h = mat.shape()[0];
    let w = mat.shape()[1];

    let mut s = 0;

    for i in 0..h {
        for j in 0..w {
            if mat[[i, j]] == 0 {
                rec1(&mut mat, [i, j]);
                s += mat.iter().filter(|&&x| (x & 0b1000_0000) != 0).count();
                for x in mat.iter_mut() {
                    *x &= 0b0111_1111;
                }
            }
        }
    }

    s.into()
}

fn rec2(mat: ArrayView2<u8>, counts: &mut Array2<u16>, pos: [usize; 2]) -> u16 {
    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    let h1 = mat[pos];

    if h1 == 9 {
        return 1;
    }

    if counts[pos] != u16::MAX {
        return counts[pos];
    }

    let mut s = 0;

    for [di, dj] in DIRS {
        let npos = [
            (pos[0] as isize + di) as usize,
            (pos[1] as isize + dj) as usize,
        ];

        if let Some(1) = mat.get(npos).map(|h2| h2.wrapping_sub(h1)) {
            s += rec2(mat, counts, npos);
        }
    }

    counts[pos] = s;

    s
}

fn part2(input: String) -> TaskResult {
    let mut mat = input_to_grid_owned(input.into_bytes());

    for x in mat.iter_mut() {
        *x = match *x {
            b'0'..=b'9' => *x - b'0',
            _ => 0b0100_0000,
        }
    }

    let h = mat.shape()[0];
    let w = mat.shape()[1];

    let mut counts = Array2::from_elem([h, w], u16::MAX);

    let mut s = 0;

    for i in 0..h {
        for j in 0..w {
            if mat[[i, j]] == 0 {
                s += rec2(mat.view(), &mut counts, [i, j]);
            }
        }
    }

    s.into()
}
