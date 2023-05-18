
pub struct LloydStrategy;
use crate::kmeans::{KmeansStrategy};
use crate::TSize;

impl<T: TSize> KmeansStrategy<T> for LloydStrategy{
    fn update(&self, data: &[T]) {
        println!("Calling rust from python!")
    }
}
