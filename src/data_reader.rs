use std::fmt::Debug;
use num::NumCast;


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
    fn read(&mut self, data: DataType<T>) where T: NumCast + Debug;
    fn get_data_ref(&self) -> &Option<Vec<T>>;
}

impl<T> DataReaderStrategy<T> for CSVReader<T> {
    fn read(&mut self, data: DataType<T>) where T: NumCast{
        match data {
            DataType::CSVData(path) => {
                println!("Reading as CSV: {}", path);
                let mut d: Vec<T> = Vec::new();
                for val in vec![1.0,1.0,1.0,1.0].iter() {
                    d.push(num::cast(*val).unwrap())
                }
                self.data = Some(d);
            },
            _ => {
                println!("CSVReader Got wrong type");
                self.data = None;
            },
        }
    }
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
}

impl<T> DataReaderStrategy<T> for NumpyReader<T> {
    fn read(&mut self, data: DataType<T>) where T: Debug {
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
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
}

