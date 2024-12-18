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

    panic!()
}

fn part2(input: String) -> TaskResult {
    todo!("{input}")
}
