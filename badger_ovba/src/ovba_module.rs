use crate::utils::decompress;

#[derive(Debug)]
pub struct OvbaModule {
    name: String,
    compressed_data: Vec<u8>,
}

impl OvbaModule {
    pub(crate) fn new(name: String, compressed_data: Vec<u8>) -> Self {
        Self {
            name,
            compressed_data,
        }
    }

    pub fn print_module(&self) {
        let data = decompress(&self.compressed_data).unwrap();

        let converted_to_string = String::from_utf8(data).unwrap();
        println!("Module Name: {}", self.name);
        println!("-------------------------------");
        println!("{}", converted_to_string);
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n")
    }
}