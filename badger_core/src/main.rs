use badger_ovba::BadgerOvba;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Badger")]
struct Opt {
    #[structopt(name = "file", long = "dig", short = "d")]
    file: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    let file = File::open(opt.file)?;
    let badger = BadgerOvba::read_from_file(file).unwrap();
    let info = badger.project_info();
    println!("Here's my references!\n{:#?}", info.project_references());
    Ok(())
}
