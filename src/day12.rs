use std::num::NonZero;

use ndarray::Array2;

use crate::{Day, TaskResult, util::input_to_grid_owned};

pub const PARTS: Day = [part1, part2];

const DIRS: [[isize; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn neighbours([i, j]: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    DIRS.into_iter().map(move |[di, dj]| {
        [(i as isize + di) as usize, (j as isize + dj) as usize]
    })
}

fn floodfill_ids(
    colors: &mut Array2<u8>,
    ids: &mut Array2<Option<NonZero<usize>>>,
    pos: [usize; 2],
    next_id: usize,
) -> usize {
    if let Some(id) = ids[pos] {
        return id.into();
    }

    let color = colors[pos];

    colors[pos] = color | 0b1000_0000;

    for npos in neighbours(pos) {
        if colors.get(npos) == Some(&color) && ids[npos].is_none() {
            ids[pos] =
                floodfill_ids(colors, ids, npos, next_id).try_into().ok();
        }
    }

    colors[pos] &= 0b0111_1111;

    if ids[pos].is_none() {
        ids[pos] = next_id.try_into().ok()
    }

    ids[pos].unwrap().into()
}

fn part1(input: String) -> TaskResult {
    let mut colors = input_to_grid_owned(input.into_bytes());
    let mut ids = Array2::from_elem(colors.dim(), None);

    let (h, w) = colors.dim();

    let mut areas = Vec::new();

    for i in 0..h {
        for j in 0..w {
            let color = colors[[i, j]];

            let id =
                floodfill_ids(&mut colors, &mut ids, [i, j], areas.len() + 1);

            if id > areas.len() {
                areas.push([0; 2]);
            }

            areas[id - 1][0] += 1;
            areas[id - 1][1] += neighbours([i, j])
                .filter(|&pos| colors.get(pos) != Some(&color))
                .count() as u64;
        }
    }

    areas.into_iter().map(|[a, p]| a * p).sum::<u64>().into()
}

fn part2(input: String) -> TaskResult {
    // let mut colors = input_to_grid_owned(input.into_bytes());
    // let mut ids = Array2::from_elem(colors.dim(), None);

    // let (h, w) = colors.dim();

    // let mut areas = Vec::new();

    // for i in 0..h {
    //     for j in 0..w {
    //         let color = colors[[i, j]];

    //         let id =
    //             floodfill_ids(&mut colors, &mut ids, [i, j], areas.len() + 1);

    //         if id > areas.len() {
    //             areas.push([0; 2]);
    //         }

    //         areas[id - 1][0] += 1;

    //         for dir in DIRS {

    //         }
    //     }
    // }

    // areas.into_iter().map(|[a, p]| a * p).sum::<u64>().into()

    todo!("{input}")
}
