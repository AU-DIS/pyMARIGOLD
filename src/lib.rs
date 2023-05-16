use cpython::{py_module_initializer, py_fn, PyNone, PyResult, Python, PyDict, PyTuple};
mod kmeans;
mod marigold;
mod lloyd;
mod kmeans_utils;
mod data_reader;

use num::Num;
use crate::data_reader::DataType;
py_module_initializer!(pyMARIGOLD, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "run", py_fn!(py, run(*args, **kwargs)))?;
    Ok(())
});
type TSize = f64;
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
    let mut runner1 = kmeans::KmeansRunner::<f64>::new(crate::marigold::MARIGOLDStrategy, crate::data_reader::CSVReader);
    runner1.set_dataset(DataType::CSVData(String::from("/path/path")));
    runner1.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner1.set_datareader(crate::data_reader::NumpyReader);
    runner1.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner1.set_dataset(DataType::CSVData(String::from("/path/path")));
        //.run();
    let mut runner2 = kmeans::KmeansRunner::<f32>::new(crate::marigold::MARIGOLDStrategy, crate::data_reader::CSVReader);
    runner2.set_dataset(DataType::CSVData(String::from("/path/path")));
    runner2.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner2.set_datareader(crate::data_reader::NumpyReader);
    runner2.set_dataset(DataType::NumpyData(vec![1.,2.,3.]));
    runner2.set_dataset(DataType::CSVData(String::from("/path/path")));

    //Aaaand back to python
    Ok(PyNone)
}