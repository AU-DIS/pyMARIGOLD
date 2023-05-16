use crate::data_reader::{CSVReader, DataReader, NumpyReader, ReaderType};
use crate::data_reader::ReaderType::CSVReader;

pub trait KmeansStrategy {
    fn update(&self);
}

pub struct KmeansRunner<T: KmeansStrategy> {
    kmeans_strategy: T,
    data_reader: DataReader,
}

impl<T: KmeansStrategy> KmeansRunner<T> {
    pub fn new(kmeans_strategy: T) -> Self {
        Self {kmeans_strategy, ..Default::default()}
    }

    pub fn add_dataset(&mut self, format: ReaderType) {
        match format {
            ReaderType::CSVReader => <DataReader as CSVReader>::read(self.data_reader),
            ReaderType::NumpyReader => <DataReader as NumpyReader>::read(self.data_reader),
        }
    }

    pub fn run(&self) -> String {
        self.kmeans_strategy.update();
        String::from("DONE")
    }
}