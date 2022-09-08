use std::fs::File;
use badger_ovba::BadgerOvba;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "Badger")]
struct Opt {
    #[structopt(name = "file", long = "dig", short = "d")]
    file: PathBuf,
}


fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    let file = File::open(opt.file)?;
    let badger = BadgerOvba::from_file(file).unwrap();
    badger.display_module();
    Ok(())
}
