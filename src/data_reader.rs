use crate::data_reader::DataTypeError::{NotCSVData, NotNumpyData};
use crate::DataType::CSVData;
use crate::TSize;
use num::NumCast;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
enum DataTypeError {
    NotCSVData,
    NotNumpyData,
}

#[derive(Debug)]
pub enum DataReaderError {
    WrongDataType(DataTypeError),
    DataNotRead,
}

impl Display for DataReaderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataReaderError::WrongDataType(e) => write!(
                f,
                "The reader was provided a wrong data formatting. {:?}",
                *e
            ),
            DataReaderError::DataNotRead => write!(f, "The dataloader has not read the data"),
        }
    }
}

pub enum DataType<T> {
    CSVData(String),
    NumpyData(Vec<T>, usize, usize),
}

pub struct CSVReader<T> {
    data: Option<Vec<T>>,
    centroids: Option<Vec<T>>,
}

impl<T> CSVReader<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            centroids: None,
        }
    }
}
pub struct NumpyReader<T> {
    data: Option<Vec<T>>,
    centroids: Option<Vec<T>>,
}
impl<T> NumpyReader<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            centroids: None,
        }
    }
}

pub trait DataReaderStrategy<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(), DataReaderError>
    where
        T: NumCast + Debug + Clone;
    fn get_data_ref(&self) -> &Option<Vec<T>>;
    fn get_centroid_ref(&self) -> &Option<Vec<T>>;
    fn get_centroid_ref_mut(&mut self) -> &mut Option<Vec<T>>;
    fn set_centroids(&mut self, c: Option<Vec<T>>);
}

impl<T> DataReaderStrategy<T> for CSVReader<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(), DataReaderError>
    where
        T: NumCast,
    {
        match data {
            DataType::CSVData(path) => {
                //TODO: All of this is Placeholder for actually loading a file
                println!("Reading as CSV: {}", path);
                let mut d: Vec<T> = Vec::new();
                for val in vec![1.0, 1.0, 1.0, 1.0].iter() {
                    d.push(num::cast(*val).unwrap())
                }
                let mut d1: Vec<T> = Vec::new();
                for val in vec![1.0, 2.0, 3.0, 4.0].iter() {
                    d1.push(num::cast(*val).unwrap())
                }
                self.data = Some(d);
                self.centroids = Some(d1);
                Ok(())
            }
            _ => {
                println!("CSVReader Got wrong type");
                self.data = None;
                self.centroids = None;
                Err(DataReaderError::WrongDataType(NotCSVData))
            }
        }
    }
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
    fn get_centroid_ref(&self) -> &Option<Vec<T>> {
        &self.centroids
    }

    fn get_centroid_ref_mut(&mut self) -> &mut Option<Vec<T>> {
        &mut self.centroids
    }
    fn set_centroids(&mut self, c: Option<Vec<T>>) {
        self.centroids = c;
    }
}

impl<T> DataReaderStrategy<T> for NumpyReader<T> {
    fn read(&mut self, data: DataType<T>) -> Result<(), DataReaderError>
    where
        T: Debug + NumCast + Clone,
    {
        match data {
            DataType::NumpyData(data_array, num_of_dimensions, num_of_centroids) => {
                println!("Reading as array {:?}", data_array);

                //TODO: Currently takes the first k points as start centroids
                self.centroids = Some(
                    data_array[0..num_of_dimensions * num_of_centroids].to_owned(), /*vec![-1., -1., 1., 1.]
                                                                                    .iter()
                                                                                    .map(|&v| num::cast(v).unwrap())
                                                                                    .collect(),*/
                );
                self.data = Some(data_array);

                Ok(())
            }
            _ => {
                println!("Wrong type");
                self.data = None;
                self.data = None;
                Err(DataReaderError::WrongDataType(NotNumpyData))
            }
        }
    }
    fn get_data_ref(&self) -> &Option<Vec<T>> {
        &self.data
    }
    fn get_centroid_ref(&self) -> &Option<Vec<T>> {
        &self.centroids
    }
    fn get_centroid_ref_mut(&mut self) -> &mut Option<Vec<T>> {
        &mut self.centroids
    }
    fn set_centroids(&mut self, c: Option<Vec<T>>) {
        self.centroids = c;
    }
}
