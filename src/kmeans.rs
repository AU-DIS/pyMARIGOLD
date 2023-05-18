use std::fmt;
use std::fmt::{Formatter,Display};
use crate::data_reader::{DataReaderStrategy, DataReaderError};
use crate::data_reader::DataReaderError::DataNotRead;
use crate::TSize;


#[derive(Debug)]
pub enum KmeansRunnerError {
    DataReader(DataReaderError),
    Kmeans,
}

impl Display for KmeansRunnerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            KmeansRunnerError::DataReader(e) =>  write!(f,"{}", e),
            KmeansRunnerError::Kmeans => write!(f, "The dataloader has not read the data"),
        }
    }
}



pub trait KmeansStrategy<T: TSize> {
    fn update(&self, data: &[T]);
}


pub struct KmeansRunner {}

impl KmeansRunner {
    pub fn run<T: TSize>(kmeans_strategy: &impl KmeansStrategy<T>, data_reader: &impl DataReaderStrategy<T>) -> Result<String,KmeansRunnerError> {
        if let Some(data) = data_reader.get_data_ref() {
            kmeans_strategy.update(data);
            Ok(String::from("DONE"))
        } else {Err(KmeansRunnerError::DataReader(DataNotRead))}
        //self.kmeans_strategy.update(self.data_reader.get_data_ref());

    }
}