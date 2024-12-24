use std::{
    fmt::{Debug, Display},
    io::{self, Write},
};

use arrayvec::ArrayVec;
use ndarray::{Array2, ArrayView2, s};
use num_traits::{AsPrimitive, PrimInt};

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

pub fn add_coords<
    A: PrimInt + AsPrimitive<B>,
    B: PrimInt + AsPrimitive<C> + 'static,
    C: PrimInt + 'static,
    const N: usize,
>(
    a: [A; N],
    b: [B; N],
) -> [C; N] {
    let mut c = [C::zero(); N];

    for ((c, &a), &b) in c.iter_mut().zip(&a).zip(&b) {
        *c = (a.as_() + b).as_()
    }

    c
}

pub fn crt(a1: i64, n1: i64, a2: i64, n2: i64) -> i64 {
    let gcd = i64::extended_gcd(&n1, &n2);
    let x = (a2 * n1 * gcd.x + a1 * n2 * gcd.y) % (n1 * n2);

    if x < 0 { x + n1 * n2 } else { x }
}

pub fn display_join<T: Display, I: Iterator<Item = T>, S: Display>(
    buf: &mut Vec<u8>,
    mut it: I,
    sep: S,
) -> Result<(), io::Error> {
    if let Some(x) = it.next() {
        write!(buf, "{x}")?;
    }

    for x in it {
        write!(buf, "{sep}{x}")?;
    }

    Ok(())
}

pub trait ConstCollect<T> {
    fn collect_const<const N: usize>(self) -> Option<[T; N]>;
}

impl<T: Debug, I: Iterator<Item = T>> ConstCollect<T> for I {
    fn collect_const<const N: usize>(self) -> Option<[T; N]> {
        self.collect::<ArrayVec<_, N>>().into_inner().ok()
    }
}
