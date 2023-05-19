
pub struct LloydStrategy;
use crate::kmeans::{KmeansStrategy};
use crate::TSize;

impl<T: TSize> KmeansStrategy<T> for LloydStrategy {
    fn run(&self, data: &[T], centroids: &[T], n: usize, d: usize, k: usize, max_iter: usize) -> T {
        todo!()
    }
    fn step(&self, data: &[T], centroids: &[T]) {
        todo!()
    }
}
