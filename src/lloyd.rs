use std::fmt::Debug;

pub struct LloydStrategy;
use crate::kmeans::{KmeansStrategy};

impl<T: num::Num + Debug> KmeansStrategy<T> for LloydStrategy{
    fn update(&self, data: &Vec<T>) {
        println!("Calling rust from python!")
    }
}
