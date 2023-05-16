use std::collections::HashMap;

pub enum ReaderType {
    CSVReader,
    NumpyReader,
}

#[derive(Default)]
pub struct DataReader {
    //accepted_formats: HashMap<String, bool>,
}
impl DataReader {
    /*fn add_format(&mut self, format: String) {
        self.accepted_formats.insert(format, true);
    }*/
}

pub trait CSVReader {
    fn read();
    //fn add_format(&self);
}
impl CSVReader for DataReader {
    fn read() {
        todo!()
    }
    /*fn add_format(&mut self) {
        self.accepted_formats.insert(String::from("csv"), true);
    }*/

}

pub trait NumpyReader {
    fn read();
    //fn add_format(&self);
}
impl NumpyReader for DataReader {
    fn read() {
        todo!()
    }
}