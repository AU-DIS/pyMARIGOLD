use std::borrow::BorrowMut;
use std::sync::Mutex;
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

pub fn recalculate<T: TSize>(
    data: &[T],
    centroids: &mut Vec<T>,
    labels: &[usize],
    d: usize,
    k: usize,
) -> bool {
    let old_centroids = &mut vec![num::cast(0.).unwrap(); k * d];
    let mut count = vec![0 as usize; k];
    centroids.clone_into(old_centroids);
    centroids.fill(num::cast(0.).unwrap());

    //Count and dist sum
    let mut mutex_cent_count: Vec<Mutex<(&mut [T], &mut usize)>> = centroids
        .chunks_mut(d)
        .zip(count.iter_mut())
        .map(|(centroid, cnt)| Mutex::new((centroid, cnt)))
        .collect();
    data.par_chunks(d)
        .zip(labels.par_iter())
        .into_par_iter()
        .for_each(|(point, label)| {
            let mut tup = mutex_cent_count[*label].lock().unwrap(); // += (*point, 1);
            point
                .iter()
                .zip(tup.0.iter_mut())
                .for_each(|(p, c_sum)| *c_sum = *p + *c_sum);
            *tup.1 += 1;
        });

    //Divide dist sum by count
    mutex_cent_count.par_iter().for_each(|mutex| {
        let mut tup = mutex.lock().unwrap();
        let cnt = tup.1.clone();
        //let c_sum = tup.0;
        tup.0
            .into_par_iter()
            .for_each(|c_sum_val| *c_sum_val = *c_sum_val / num::cast(cnt).unwrap())
    });

    //Did this update centroids? Find out in the next episode of Rust is pain

    //Calculate change in centroids
    let converged = true;

    converged
}

#[cfg(test)]
mod tests;
