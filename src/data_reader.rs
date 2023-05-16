use std::collections::HashMap;
use std::fmt::Debug;
use num::Num;

pub enum DataType<T: num::Num> {
    CSVData(String),
    NumpyData(Vec<T>),
}

pub struct CSVReader<T: num::Num + Debug>{
    data: Option<Vec<T>>
}

impl<T: num::Num + Debug> CSVReader<T> {
    pub fn new() -> Self{
        Self {data: None}
    }

}
pub struct NumpyReader<T: num::Num + Debug>{
    data: Option<Vec<T>>
}
impl<T: num::Num + Debug> NumpyReader<T> {
    pub fn new() -> Self{
        Self {data: None}
    }
}


pub trait DataReaderStrategy<T: num::Num + Debug> {
    fn read(self: &Self, data: DataType<T>);
    fn get_data_ref(self: &Self) -> &Option<Vec<T>>;
}

impl<T: num::Num + Debug> DataReaderStrategy<T> for CSVReader<T> {
    fn read(self: &mut Self, data: DataType<T>) {
        match data {
            DataType::CSVData(path) => {
                println!("Reading as CSV: {}", path);
                self.data = Some(vec![1.0,1.0,1.0,1.0]);
            },
            _ => {
                println!("CSVReader Got wrong type");
                self.data = None;
            },
        }
    }
    fn get_data_ref(self: &Self) -> &Option<Vec<T>> {
        &self.data
    }
}

impl<T: num::Num + Debug> DataReaderStrategy<T> for NumpyReader<T> {
    fn read(self: &mut Self, data: DataType<T>)  {
        match data {
            DataType::NumpyData(data_array) => {
                println!("Reading as array {:?}",data_array);
                self.data = Some(data_array);
            },
            _ => { println!("Wrong type");
                self.data = None;
            },
        }
    }
    fn get_data_ref(self: &Self) -> &Option<Vec<T>> {
        &self.data
    }
}

