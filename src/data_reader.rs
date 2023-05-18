use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use num::NumCast;
use crate::data_reader::DataTypeError::{NotCSVData, NotNumpyData};
use crate::DataType::CSVData;

#[derive(Debug)]
enum DataTypeError {
    NotCSVData,
    NotNumpyData
}

#[derive(Debug)]
pub enum DataReaderError {
    WrongDataType(DataTypeError),
    DataNotRead
}

impl Display for DataReaderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataReaderError::WrongDataType(e) => write!(f, "The reader was provided a wrong data formatting. {:?}", *e),
            DataReaderError::DataNotRead => write!(f, "The dataloader has not read the data"),
        }
    }
}

pub enum DataType<T> {
    CSVData(String),
    NumpyData(Vec<T>),
}

pub struct CSVReader<T>{
    data: Option<Vec<T>>
}

impl<T> CSVReader<T> {
    pub fn new() -> Self{
        Self {data: None}
    }

}
pub struct NumpyReader<T>{
    data: Option<Vec<T>>
}
impl<T> NumpyReader<T> {
    pub fn new() -> Self{
        Self {data: None}
    }
}


pub trait DataReaderStrategy<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(), DataReaderError> where T: NumCast + Debug;
    fn get_data_ref(&self) -> &Option<Vec<T>>;
}

impl<T> DataReaderStrategy<T> for CSVReader<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(),DataReaderError> where T: NumCast{
        match data {
            DataType::CSVData(path) => {
                println!("Reading as CSV: {}", path);
                let mut d: Vec<T> = Vec::new();
                for val in vec![1.0, 1.0, 1.0, 1.0].iter() {
                    d.push(num::cast(*val).unwrap())
                }
                self.data = Some(d);
                Ok(())
            },
            _ => {
                println!("CSVReader Got wrong type");
                self.data = None;
                Err(DataReaderError::WrongDataType(NotCSVData))
            },
        }
    }
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
}

impl<T> DataReaderStrategy<T> for NumpyReader<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(),DataReaderError> where T: Debug {
        match data {
            DataType::NumpyData(data_array) => {
                println!("Reading as array {:?}",data_array);
                self.data = Some(data_array);
                Ok(())
            },
            _ => { println!("Wrong type");
                self.data = None;
                Err(DataReaderError::WrongDataType(NotNumpyData))
            },
        }
    }
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
}

