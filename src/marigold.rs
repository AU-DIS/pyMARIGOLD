use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};
use crate::TSize;

pub struct MARIGOLDStrategy;

impl<T: TSize> KmeansStrategy<T> for  MARIGOLDStrategy  {
    fn run(&self, data: &[T], centroids: &[T], n: usize, d: usize, k: usize, max_iter: usize) -> T {
        let p = euclidian_distance(data, &centroids);
        //let data1: Vec<T> = vec![1.,1.,1.,1.];
        //let data2: Vec<T> = vec![2.,2.,2.,2.];
        //let v = euclidian_distance(data, data);

        println!("Here is a value from far away {:?}", p);
        p
    }
    fn step(&self, data: &[T], centroids: &[T]) {
        todo!()
    }
}

#[cfg(test)]
mod tests;
