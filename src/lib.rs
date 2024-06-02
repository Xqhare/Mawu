mod utils;
pub mod errors;
pub mod mawu_values;

mod csv {
    use std::path::Path;

    use crate::{errors::MawuError, mawu_values::MawuValue};

    pub fn from_csv_headed<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        todo!()
    }

    pub fn from_csv_headless<T: AsRef<Path>>(path: T) -> Result<MawuValue, MawuError> {
        todo!()
    }
}


pub fn testing() {
    println!("Hello, world!");
    let t = utils::file_handling::read_file("README.md").unwrap();
    println!("{}", t);
}
