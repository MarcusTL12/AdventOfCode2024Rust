use ndarray::Array2;
use priority_queue::PriorityQueue;

use crate::{
    Day, TaskResult,
    util::{add_coords, input_to_grid_owned},
};

pub const PARTS: Day = [part1, part2];

const DIRS: [[isize; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn part1(input: String) -> TaskResult {
    let mut grid = input_to_grid_owned(input.into());

    let [start, stop] = grid.indexed_iter_mut().fold(
        [[0; 2]; 2],
        |[mut start, mut stop], ((i, j), x)| {
            match *x {
                b'S' => {
                    start = [i, j];
                    *x = 0;
                }
                b'E' => {
                    stop = [i, j];
                    *x = 0;
                }
                b'.' => *x = 0,
                b'#' => *x = 1,
                _ => panic!(),
            }
            [start, stop]
        },
    );

    let mut pq = PriorityQueue::new();
    pq.push((start, 0), 0);

    while let Some(((pos, dir), score)) = pq.pop() {
        let dirflag = 1 << (dir + 1);
        grid[pos] |= dirflag;

        if pos == stop {
            return (-score).into();
        }

        let npos = add_coords(pos, DIRS[dir]);
        if grid[npos] & (1 | dirflag) == 0 {
            pq.push_increase((npos, dir), score - 1);
        }

        for dir_off in [1, 3] {
            let ndir = (dir + dir_off) % 4;
            let ndirflag = 1 << (ndir + 1);
            if grid[pos] & (1 | ndirflag) == 0 {
                pq.push_increase((pos, ndir), score - 1000);
            }
        }
    }

    panic!("Did not find exit!")
}

fn part2(input: String) -> TaskResult {
    let mut grid = input_to_grid_owned(input.into());

    let [start, stop] = grid.indexed_iter_mut().fold(
        [[0; 2]; 2],
        |[mut start, mut stop], ((i, j), x)| {
            match *x {
                b'S' => {
                    start = [i, j];
                    *x = 0;
                }
                b'E' => {
                    stop = [i, j];
                    *x = 0;
                }
                b'.' => *x = 0,
                b'#' => *x = 1,
                _ => panic!(),
            }
            [start, stop]
        },
    );

    let mut distmap = Array2::from_elem(grid.dim(), [0u32; 4]);
    let mut backtracker = Array2::from_elem(grid.dim(), 0u8);

    let mut pq = PriorityQueue::new();
    pq.push((start, 0, 0), 0i64);

    while let Some(((pos, dir, from), score)) = pq.pop() {
        let dirflag = 1 << (dir + 1);

        if grid[pos] & dirflag == 0 || distmap[pos][dir] == (-score) as u32 {
            distmap[pos][dir] = (-score) as u32;
            backtracker[pos] |= from;
        }

        grid[pos] |= dirflag;

        if pos == stop {
            break;
        }

        let npos = add_coords(pos, DIRS[dir]);
        if grid[npos] & (1 | dirflag) == 0 {
            pq.push_increase((npos, dir, dirflag >> 1), score - 1);
        }

        for dir_off in [1, 3] {
            let ndir = (dir + dir_off) % 4;
            let ndirflag = 1 << (ndir + 1);
            if grid[pos] & (1 | ndirflag) == 0 {
                pq.push_increase((pos, ndir, from), score - 1000);
            }
        }
    }

    backtrack_count(&mut backtracker, stop).into()
}

fn backtrack_count(backtracker: &mut Array2<u8>, pos: [usize; 2]) -> usize {
    let dirs = backtracker[pos];

    if (dirs >> 4) & 1 == 1 {
        return 0;
    }

    backtracker[pos] |= 1 << 4;

    1 + DIRS
        .into_iter()
        .enumerate()
        .filter(|(i, _)| (dirs >> ((i + 2) % 4)) & 1 == 1)
        .map(|(_, dir)| backtrack_count(backtracker, add_coords(pos, dir)))
        .sum::<usize>()
}
