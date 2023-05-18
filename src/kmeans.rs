
use crate::data_reader::{DataReaderStrategy, DataType};
use crate::TSize;


pub trait KmeansStrategy<T: TSize> {
    fn update(&self, data: &[T]);
}

pub struct KmeansRunner<T: TSize> {

    kmeans_strategy: Box<dyn KmeansStrategy<T>>,
    data_reader: Box<dyn DataReaderStrategy<T>>,
}

impl<T: TSize> KmeansRunner<T> {
    pub fn new(kmeans_strategy: impl KmeansStrategy<T> + 'static, data_reader: impl DataReaderStrategy<T> +'static) -> Self {
        Self {kmeans_strategy: Box::new(kmeans_strategy), data_reader: Box::new(data_reader)}
    }

    pub fn set_datareader(&mut self, reader: impl DataReaderStrategy<T> + 'static) {
        self.data_reader = Box::new(reader);
    }
    pub fn set_dataset(&mut self, data: DataType<T>) {
        self.data_reader.read(data);
    }

    pub fn run(&self) -> String {
        if let Some(data) = self.data_reader.get_data_ref() {
            self.kmeans_strategy.update(data);
            //let v = self.data_reader.get_data_ref();

            return String::from("DONE");
        }
        //self.kmeans_strategy.update(self.data_reader.get_data_ref());
        String::from("FAIL")
    }
}