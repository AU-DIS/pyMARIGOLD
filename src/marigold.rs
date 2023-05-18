use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};
use crate::TSize;

pub struct MARIGOLDStrategy;

impl<T: TSize> KmeansStrategy<T> for  MARIGOLDStrategy  {
    fn update(&self, data: &[T])
    {
        let data2: Vec<T> = vec![2.,2.,2.,2.].iter().map(|&v| num::cast(v).unwrap()).collect();
        let p = euclidian_distance(data, &data2);
        //let data1: Vec<T> = vec![1.,1.,1.,1.];
        //let data2: Vec<T> = vec![2.,2.,2.,2.];
        //let v = euclidian_distance(data, data);

        println!("Here is a value from far away {:?}", p);

    }
}
