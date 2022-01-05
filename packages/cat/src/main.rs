use cat::lib::read_file::read_file;
use core::panic;
use std::env;
use std::io::{stdout, BufWriter, Write};
fn main() {
    let input_args: Vec<String> = env::args().collect();

    match input_args.len() {
        1 => panic!("No arguments provided. input args= {:?}", input_args),
        _ => {
            input_args.iter().enumerate().for_each(|(i, arg)| {
                if i >= 1 {
                    let read_txt = read_file(&arg);
                    match read_txt {
                        Ok(txt) => {
                            let out = stdout();
                            let mut writer = BufWriter::new(out.lock());
                            writer.write_all(txt.as_bytes()).unwrap();
                        }
                        Err(e) => panic!("Error reading file: {:?}", e),
                    }
                }
            });
        }
    }
}
