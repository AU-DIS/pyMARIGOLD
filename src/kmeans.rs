use std::fmt::Debug;
use std::option::Option;
use num::NumCast;
use simd_euclidean::Vectorized;
use crate::data_reader::{CSVReader, DataReaderStrategy, NumpyReader, DataType};



pub trait KmeansStrategy<T> {
    fn update<'a, 'b:'a>(&'a self, data: &'b Vec<T>) where
    &'b Vec<T>: Vectorized,
    <&'b Vec<T> as Vectorized>::Output: Debug;
}


pub struct KmeansRunner<T> {

    kmeans_strategy: Box<dyn KmeansStrategy<T>>,
    data_reader: Box<dyn DataReaderStrategy<T>>,
}

impl<'b, T> KmeansRunner<T> where
    T: 'b + NumCast + Debug,
    &'b Vec<T>: Vectorized,
    <&'b Vec<T> as Vectorized>::Output: Debug
{
    pub fn new(kmeans_strategy: impl KmeansStrategy<T> + 'static, data_reader: impl DataReaderStrategy<T> +'static) -> Self {
        Self {kmeans_strategy: Box::new(kmeans_strategy), data_reader: Box::new(data_reader)}
    }

    pub fn set_datareader(&mut self, reader: impl DataReaderStrategy<T> + 'static) {
        self.data_reader = Box::new(reader);
    }
    pub fn set_dataset(&mut self, data: DataType<T>) {
        self.data_reader.read(data);
    }

    pub fn run(&'b self) -> String {
        if let Some(data) = self.data_reader.get_data_ref() {
            self.kmeans_strategy.update(data);
            //let v = self.data_reader.get_data_ref();

            return String::from("DONE");
        }
        //self.kmeans_strategy.update(self.data_reader.get_data_ref());
        return String::from("FAIL")
    }
}