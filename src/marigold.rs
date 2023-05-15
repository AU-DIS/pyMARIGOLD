use num::Rational64;
use num::Num;
use crate::kmeans_utils::euclidian_distance;
use crate::kmeans::{KmeansStrategy};
pub struct MARIGOLDStrategy;

//int_trait_impl!(Num for usize u8 u16 u32 u64 isize i8 i16 i32 i64);
impl KmeansStrategy for MARIGOLDStrategy {
    fn update(&self) {
        let v = euclidian_distance(&vec![1.,1.,1.,1.] as &Vec<f64>, &vec![2.,2.,2.,2.] as &Vec<f64>);
        println!("Here is a value from far away {}", v);
    }
}
