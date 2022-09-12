#[derive(Debug, Clone)]
pub struct OvbaModule {
    name: String,
    code: Vec<u8>,
}

impl OvbaModule {
    pub(crate) fn new(name: String, code: Vec<u8>) -> Self {
        Self { name, code }
    }

    pub fn print_module(&self) {
        let converted_to_string = String::from_utf8(self.code.clone()).unwrap();
        println!("Module Name: {}", self.name);
        println!("-------------------------------");
        println!("{}", converted_to_string);
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n")
    }
}
