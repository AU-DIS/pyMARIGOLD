use cpython::{py_module_initializer, py_fn, PyNone, PyResult, Python, PyDict, PyTuple};
mod kmeans;
mod marigold;
mod lloyd;
mod kmeans_utils;

use num::Num;
py_module_initializer!(pyMARIGOLD, |py, m| {
    m.add(py, "__doc__", "Module documentation string")?;
    m.add(py, "run", py_fn!(py, run(*args, **kwargs)))?;
    Ok(())
});


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
    kmeans::KmeansRunner::new(crate::marigold::MARIGOLDStrategy).run();

    //Aaaand back to python
    Ok(PyNone)
}