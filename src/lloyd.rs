use std::fmt::Debug;

pub struct LloydStrategy;
use crate::kmeans::{KmeansStrategy};

impl<T> KmeansStrategy<T> for LloydStrategy{
    fn update<'a, 'b: 'a>(&self, data: &Vec<T>) where T: 'b, {
        println!("Calling rust from python!")
    }
}
