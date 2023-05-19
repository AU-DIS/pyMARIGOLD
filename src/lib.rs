use cpython::{py_fn, py_module_initializer, PyDict, PyNone, PyResult, PyTuple, Python};
use num::{Float, NumCast};
use std::fmt::Debug;
use std::iter::Sum;

//use log::warn;
mod data_reader;
mod kmeans;
mod kmeans_utils;
mod lloyd;
mod marigold;

use crate::data_reader::{CSVReader, DataReaderStrategy, DataType, NumpyReader};
py_module_initializer!(pyMARIGOLD, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "run", py_fn!(py, run(*args, **kwargs)))?;
    Ok(())
});
//type TSize = f64;
pub trait TSize: Float + Debug + Sum + NumCast + Sync + Send {}

impl TSize for f64 {}
impl TSize for f32 {}

fn run(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyNone> {
    //We can parse arguments from python here
    for arg in args.iter(py) {
        println!("Rust got {}", arg);
    }
    if let Some(kwargs) = kwargs {
        for (key, val) in kwargs.items(py) {
            println!("{} = {}", key, val);
        }
    }
    //And now we actually do something with it
    //let runner1 = kmeans::KmeansRunner::run(); // ::new(crate::marigold::MARIGOLDStrategy, crate::data_reader::CSVReader::new());
    let mut reader1 = NumpyReader::<f64>::new();
    reader1.read(DataType::NumpyData(vec![1., 2., 3.])).unwrap();
    let kmeans1 = marigold::MARIGOLDStrategy;
    let runner = kmeans::KmeansRunner::new(3, 1, 2, 100);
    let mut run_result = runner
        .run(&kmeans1, &reader1)
        .expect("Could not run Kmeans");
    println!("{}", run_result);

    let mut reader2 = NumpyReader::<f32>::new();
    //reader2.read(DataType::CSVData(String::from("/path"))).expect("DataReader failed to read data");
    reader2
        .read(DataType::NumpyData(vec![1., 2., 3.]))
        .expect("DataReader failed to read data");
    run_result = runner
        .run(&kmeans1, &reader2)
        .expect("Could not run Kmeans");
    println!("{}", run_result);

    //runner1.set_dataset(DataType::CSVData(String::from("/path/path")));
    //runner1.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    //runner1.set_datareader(crate::data_reader::NumpyReader::new());
    //runner1.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    //runner1.set_dataset(DataType::CSVData(String::from("/path/path")));
    //runner1.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));

    //.run();
    /*let mut runner2 = kmeans::KmeansRunner::<f32>::new(crate::marigold::MARIGOLDStrategy, crate::data_reader::CSVReader::new());
    runner2.set_dataset(DataType::CSVData(String::from("/path/path")));
    runner2.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner2.set_datareader(crate::data_reader::NumpyReader::new());
    runner2.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner2.set_dataset(DataType::CSVData(String::from("/path/path")));
    runner2.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));*/
    //println!("{}", runner2.run());

    //Aaaand back to python
    Ok(PyNone)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_test() {
        assert!(true);
    }
}
