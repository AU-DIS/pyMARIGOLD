use crate::data_reader::DataReaderError::DataNotRead;
use crate::data_reader::{DataReaderError, DataReaderStrategy};
use crate::TSize;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KmeansRunnerError {
    DataReader(DataReaderError),
    CentroidsNone,
    DataNone,
    Kmeans,
}

impl Display for KmeansRunnerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            KmeansRunnerError::DataReader(e) => write!(f, "{}", e),
            KmeansRunnerError::CentroidsNone => {
                write!(f, "Got data reference, but not for centroids")
            }
            KmeansRunnerError::DataNone => write!(f, "Got centroids reference, but not for data"),
            KmeansRunnerError::Kmeans => write!(f, "The dataloader has not read the data"),
        }
    }
}

pub trait KmeansStrategy<T: TSize> {
    fn run(
        &self,
        data: &[T],
        centroids: &mut Vec<T>,
        num_data_points: usize,
        num_dimensions: usize,
        num_centroids: usize,
        max_iter: usize,
    ) -> Box<[usize]>;
    fn step(&self, data: &[T], centroids: &mut Vec<T>, labels: &mut [usize], d: usize, k: usize);
}

pub struct KmeansRunner {
    num_data_points: usize,
    num_dimension: usize,
    num_centroids: usize,
    max_iter: usize,
}
/*impl Default for KmeansRunner {
    fn default() -> Self {
        Self { num_data_points: None, num_dimension: None, num_centroids: None, max_iter: 100}
    }
}*/

impl KmeansRunner {
    pub fn new(
        num_data_points: usize,
        num_dimension: usize,
        num_centroids: usize,
        max_iter: usize,
    ) -> Self {
        Self {
            num_data_points,
            num_dimension,
            num_centroids,
            max_iter,
        }
    }
    pub fn run<T: TSize>(
        &self,
        kmeans_strategy: &impl KmeansStrategy<T>,
        data_reader: &mut impl DataReaderStrategy<T>,
    ) -> Result<Box<[usize]>, KmeansRunnerError> {
        let mut c = std::mem::take(data_reader.get_centroid_ref_mut()); //TODO: Is this insane or actually how it should be done?
        match (data_reader.get_data_ref(), &mut c) {
            (Some(data), Some(centroids)) => {
                let labels = kmeans_strategy.run(
                    data,
                    centroids,
                    self.num_data_points,
                    self.num_dimension,
                    self.num_centroids,
                    self.max_iter,
                );
                data_reader.set_centroids(c); //TODO: Is this insane or actually how it should be done?
                Ok(labels)
            }
            (None, Some(_)) => Err(KmeansRunnerError::DataNone),
            (Some(_), None) => Err(KmeansRunnerError::CentroidsNone),
            (None, None) => Err(KmeansRunnerError::DataReader(DataNotRead)),
        }
        //self.kmeans_strategy.update(self.data_reader.get_data_ref());
    }
}
