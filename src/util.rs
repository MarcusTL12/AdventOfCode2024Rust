use ndarray::{Array2, ArrayView2, s};

use num_integer::Integer;

fn linelen(input: &[u8]) -> usize {
    input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap()
}

pub fn input_to_grid(input: &[u8]) -> ArrayView2<u8> {
    let w = linelen(input);
    let h = input.len() / (w + 1);

    ArrayView2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}

pub fn input_to_grid_owned(input: Vec<u8>) -> Array2<u8> {
    let w = linelen(&input);
    let h = input.len() / (w + 1);

    Array2::from_shape_vec([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}

pub fn crt(a1: i64, n1: i64, a2: i64, n2: i64) -> i64 {
    let gcd = i64::extended_gcd(&n1, &n2);
    let x = (a2 * n1 * gcd.x + a1 * n2 * gcd.y) % (n1 * n2);

    if x < 0 { x + n1 * n2 } else { x }
}
