use rayon::prelude::*;
pub struct LloydStrategy;

use crate::kmeans::KmeansStrategy;
use crate::kmeans_utils::squared_distance;
use cpython::py_class::slots::type_error_to_false;
use num::NumCast;
//use crate::lloyd::ConvergeState::Converged;
use crate::TSize;

impl<T: TSize> KmeansStrategy<T> for LloydStrategy {
    fn run(
        &self,
        data: &[T],
        centroids: &[T],
        n: usize,
        d: usize,
        k: usize,
        max_iter: usize,
    ) -> Box<[usize]> {
        let mut labels = vec![0; n];
        let mut iter = 0;
        let mut converged: bool = false;

        while iter < max_iter && !converged {
            self.step(data, centroids, &mut labels, d, k); //Step (Calculate distance + update + recalculate)
            iter += 1;
        }

        Box::from(labels)
    }
    fn step(&self, data: &[T], centroids: &[T], mut labels: &mut [usize], d: usize, k: usize) {
        //TODO: Help. It hurts.
        data.par_chunks(d)
            .zip(labels.par_iter_mut())
            .for_each(|(point, label)| {
                let mut distances: Vec<T> = vec![num::cast(0.).unwrap(); k];
                centroids
                    .par_chunks(d)
                    .zip(distances.par_iter_mut())
                    .for_each(|(centroid, dist)| *dist = squared_distance(point, centroid));
                distances.iter().enumerate().for_each(|(j, dist)| {
                    if *dist < distances[*label] {
                        *label = j
                    }
                });
            });

        /*data.par_chunks(d).enumerate().for_each(|(i, point)| {
            let mut distances: Vec<T> = vec![num::cast(0.).unwrap();k];
            centroids.par_chunks(d).enumerate()
                .for_each(|(j,centroid)| distances[j] = squared_distance(point, centroid));
            let distances = distances;
            //let mut labels = labels;
            distances.par_iter().enumerate().for_each(|(j, dist)| if *dist < distances[labels[j]] {labels[i]=j});

        });*/
    }
}
