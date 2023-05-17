use std::fmt::Debug;
use num::Rational64;
use num::Num;
use simd_euclidean::Vectorized;
use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};

pub struct MARIGOLDStrategy;

impl<T: num::Num + Debug> KmeansStrategy<T> for  MARIGOLDStrategy  {
    fn update<'a, 'b: 'a>(&'a self, data: &'b Vec<T>) where &'b Vec<T>: Vectorized, <&'b Vec<T> as Vectorized>::Output: Debug, T: 'b{
        //let data1: Vec<T> = vec![1.,1.,1.,1.];
        //let data2: Vec<T> = vec![2.,2.,2.,2.];
        let v = euclidian_distance(data, data);
        println!("Here is a value from far away {:?}", v);
    }
}
