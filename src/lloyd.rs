use rayon::prelude::*;
pub struct LloydStrategy;

use crate::kmeans::KmeansStrategy;
use crate::kmeans_utils::{squared_distance, recalculate};
use cpython::py_class::slots::type_error_to_false;
use num::NumCast;
//use crate::lloyd::ConvergeState::Converged;
use crate::TSize;

impl<T: TSize> KmeansStrategy<T> for LloydStrategy {
    fn run(
        &self,
        data: &[T],
        centroids:  &[T],
        n: usize,
        d: usize,
        k: usize,
        max_iter: usize,
    ) -> Box<[usize]> {
        let mut labels = vec![0; n];
        let mut iter = 0;
        let mut converged: bool = false;

        while iter < max_iter && !converged {
            self.step(data, centroids, &mut labels, d, k); //Step (Calculate distance + update)
            recalculate(data, &mut centroids.to_vec(), &*labels, d, k);//Recalculate TODO: to_vec is no no
            iter += 1;
        }

        Box::from(labels)
    }
    fn step(&self, data: &[T], centroids: &[T], mut labels: &mut [usize], d: usize, k: usize) {
        //TODO: Help. It hurts.
        //For all datapoints
        data.par_chunks(d) //TODO: The Data is always viewed in chunks, so I can use this as the "data" param.
            .zip(labels.par_iter_mut())
            .for_each(|(point, label)| {
                //For all centroids
                let mut distances: Vec<T> = vec![num::cast(0.).unwrap(); k]; //TODO: Surely I can move this out. (With Vec<Mutex<_>>)
                centroids
                    .par_chunks(d)
                    .zip(distances.par_iter_mut())
                    .for_each(|(centroid, dist)| *dist = squared_distance(point, centroid));

                //Update label
                distances.iter().enumerate().for_each(|(j, dist)| {
                    if *dist < distances[*label] {
                        *label = j
                    }
                });
            });
    }
}
