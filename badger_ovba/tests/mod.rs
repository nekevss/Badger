#[cfg(test)]

use badger_ovba::BadgerOvba;
use std::fs::File;
use std::result::Result;
use std::io::Error;

#[test]
fn ovba_from_file() -> Result<(), badger_ovba::error::Error> {
    let mut file = File::open("../examples/hello_world.bin").unwrap();
    BadgerOvba::from_file(file)?;
    Ok(())
}
