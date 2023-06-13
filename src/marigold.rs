use crate::kmeans::KmeansStrategy;
use crate::kmeans_utils::euclidian_distance;
use crate::TSize;

pub struct MARIGOLDStrategy;

impl<T: TSize> KmeansStrategy<T> for MARIGOLDStrategy {
    fn run(
        &self,
        data: &[T],
        centroids: &mut Vec<T>,
        n: usize,
        d: usize,
        k: usize,
        max_iter: usize,
    ) -> Box<[usize]> {
        todo!()
        /*let mut labels  = [0, n];
        let p = euclidian_distance(data, &centroids);
        //let data1: Vec<T> = vec![1.,1.,1.,1.];
        //let data2: Vec<T> = vec![2.,2.,2.,2.];
        //let v = euclidian_distance(data, data);

        println!("Here is a value from far away {:?}", p);
        Box::from(labels)*/
    }
    fn step(&self, data: &[T], centroids: &mut Vec<T>, mut labels: &mut [usize], d: usize, k: usize) {
        todo!()
    }
}

//#[cfg(test)]
//mod tests;
