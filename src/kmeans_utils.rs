//use simd_euclidean::Vectorized;
use rayon::prelude::*;

// Generic Vectorized is implicit a reference. Not confusing at all.

use crate::TSize;

pub fn euclidian_distance<T: TSize>(data1: &[T], data2: &[T]) -> T {
    T::sqrt(squared_distance(data1, data2))
}

pub fn squared_distance<T: TSize>(data1: &[T], data2: &[T]) -> T {
    data1
        .par_iter()
        .zip(data2.par_iter())
        .map(|(&x, &y)| (x - y) * (x - y))
        .sum()
}

#[cfg(test)]
mod tests;
