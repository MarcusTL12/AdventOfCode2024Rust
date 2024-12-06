use ndarray::{Array2, ArrayView2, s};

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

pub fn input_to_grid_owned_with_newline(input: Vec<u8>) -> Array2<u8> {
    let w = linelen(&input);
    let h = input.len() / (w + 1);

    Array2::from_shape_vec([h, w + 1], input).unwrap()
}
