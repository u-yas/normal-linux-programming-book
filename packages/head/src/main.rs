use std::io::Write;

// Note: this requires the `derive` feature
use clap;
use clap::Parser;
use head::lib::read::read_file;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// output file
    #[clap(name = "file", parse(from_os_str))]
    file: Option<std::path::PathBuf>,

    /// number of lines to output
    #[clap(short, long, default_value_t = 1)]
    lines: u16,
}

fn main() {
    let args = Args::parse();

    match args.file {
        Some(file) => {
            if !file.is_file() {
                eprintln!("File does not exist");
                std::process::exit(1);
            }
            let file_to_str = file.to_str().unwrap();
            let result = read_file(&file_to_str, args.lines);
            match result {
                Ok(txt) => {
                    let out = std::io::stdout();
                    let mut writer = std::io::BufWriter::new(out.lock());
                    writer.write_all(txt.as_bytes()).unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading file: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        None => panic!("No file specified"),
    }
}
