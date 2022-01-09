use clap::Parser;
use std::fs::create_dir_all;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// output file
    #[clap(name = "dir_path", parse(from_os_str))]
    dir_path: Option<std::path::PathBuf>,

    /// number of lines to output
    #[clap(short, long, default_value_t = 1)]
    lines: u16,
}

fn main() {
    let args = Args::parse();

    let dir_path = match args.dir_path {
        Some(dir_path) => dir_path,
        None => panic!("No directory specified"),
    };

    match create_dir_all(dir_path) {
        Ok(_) => {}
        Err(_) => todo!(),
    };
}
