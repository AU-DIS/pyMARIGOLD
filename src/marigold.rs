use num::Rational64;
use num::Num;
use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};

pub struct MARIGOLDStrategy;

impl KmeansStrategy for MARIGOLDStrategy {
    fn update(&self) {
        let v = euclidian_distance(&vec![1.,1.,1.,1.] as &Vec<f64>, &vec![2.,2.,2.,2.] as &Vec<f64>);
        println!("Here is a value from far away {}", v);
    }
}
