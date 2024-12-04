use ndarray::{ArrayView2, s};

pub fn input_to_grid(input: &[u8]) -> ArrayView2<u8> {
    let w = input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = input.len() / (w + 1);

    ArrayView2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}
