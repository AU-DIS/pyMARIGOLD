use std::fmt::Debug;
use std::option::Option;
use simd_euclidean::Vectorized;
use crate::data_reader::{CSVReader, DataReaderStrategy, NumpyReader, DataType};



pub trait KmeansStrategy<T: num::Num + Debug> {
    fn update<'a, 'b:'a>(&'a self, data: &'b Vec<T>) where &'b Vec<T>: Vectorized, <&'b Vec<T> as Vectorized>::Output: Debug, T: 'b;
}


pub struct KmeansRunner<T: num::Num + Debug> {

    kmeans_strategy: Box<dyn KmeansStrategy<T>>,
    data_reader: Box<dyn DataReaderStrategy<T>>,
}

impl<'a, 'b:'a, T: num::Num + Debug + 'b> KmeansRunner<T> where &'a Vec<T>: Vectorized, <&'a Vec<T> as Vectorized>::Output: Debug {
    pub fn new(kmeans_strategy: impl KmeansStrategy<T> +'a +'static, data_reader: impl DataReaderStrategy<T> + 'b +'static) -> Self {
        Self {kmeans_strategy: Box::new(kmeans_strategy), data_reader: Box::new(data_reader)}
    }

    pub fn set_datareader(&mut self, reader: impl DataReaderStrategy<T> + 'b + 'static) {
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