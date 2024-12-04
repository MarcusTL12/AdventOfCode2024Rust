use crate::{Day, TaskResult, util::input_to_grid};

pub const PARTS: Day = [part1, part2];

fn part1(input: String) -> TaskResult {
    let mat = input_to_grid(input.as_bytes());

    let dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
        [1, 1],
        [-1, 1],
        [1, -1],
        [-1, -1],
    ];

    let h = mat.shape()[0];
    let w = mat.shape()[1];

    let mut c = 0;

    for i in 0..h {
        for j in 0..w {
            for [dx, dy] in dirs {
                if (0..4)
                    .scan([-dx, -dy], |[x, y], _| {
                        *x += dx;
                        *y += dy;

                        mat.get([
                            (i as isize + *x) as usize,
                            (j as isize + *y) as usize,
                        ])
                        .or(Some(&b' '))
                    })
                    .zip(b"XMAS")
                    .all(|(a, b)| a == b)
                {
                    c += 1;
                }
            }
        }
    }

    TaskResult::from(c)
}

fn part2(input: String) -> TaskResult {
    let mat = input_to_grid(input.as_bytes());

    let h = mat.shape()[0];
    let w = mat.shape()[1];

    let mut c = 0;

    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if mat[[i, j]] == b'A' {
                let mut v1 = [mat[[i - 1, j - 1]], mat[[i + 1, j + 1]]];
                let mut v2 = [mat[[i + 1, j - 1]], mat[[i - 1, j + 1]]];

                v1.sort();
                v2.sort();

                if v1 == *b"MS" && v2 == *b"MS" {
                    c += 1;
                }
            }
        }
    }

    TaskResult::from(c)
}
