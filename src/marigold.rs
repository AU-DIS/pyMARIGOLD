use std::fmt::Debug;
use num::Rational64;
use num::Num;
use simd_euclidean::Vectorized;
use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};

pub struct MARIGOLDStrategy;

impl<'a, T: num::Num + Debug + 'a> KmeansStrategy<T> for MARIGOLDStrategy where &'a Vec<T>: Vectorized, <&'a Vec<T> as Vectorized>::Output: Debug {
    fn update(&self, data: &Vec<T>) {
        //let data1: Vec<T> = vec![1.,1.,1.,1.];
        //let data2: Vec<T> = vec![2.,2.,2.,2.];
        let v = euclidian_distance(data, data);
        println!("Here is a value from far away {:?}", v);
    }
}
