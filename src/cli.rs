use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "Le Mutate", about = "Randomize a level using WFC algorithm.")]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short = "f", long = "folder", default_value = "$_out")]
    pub folder: String,

    #[structopt(short = "n", long = "name", default_value = "$_new%.json")]
    pub name: String,

    #[structopt(short = "c", long = "count", default_value = "1")]
    pub count: u8,

    #[structopt(short = "d", long = "dimensions", help = " [default: Same as input.]")]
    pub dimensions: Option<Vec<usize>>,
}